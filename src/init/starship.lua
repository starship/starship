if (clink.version_encoded or 0) < 10020030 then
  error("Starship requires a newer version of Clink; please upgrade to Clink v1.2.30 or later.")
end

local STARSHIP = [[::STARSHIP::]]
local STREAMING_CLINK = (clink.version_encoded or 0) >= 10080000
local POLL_INTERVAL = 0.02
local TARGETS = {
  { name = "left", flag = "", prompt = "" },
  { name = "right", flag = " --right", prompt = "" },
}

local starship_prompt = clink.promptfilter(5)
local current_session
local generation = 0
local start_time = os.clock()
local command_duration = 0
local is_line_empty = true

local function shell_arguments()
  return " --status=" .. os.geterrorlevel()
    .. " --cmd-duration=" .. math.floor(command_duration * 1000)
    .. " --terminal-width=" .. console.getwidth()
    .. " --keymap=" .. (rl.getvariable("keymap") or "emacs-standard")
end

local function render(target, arguments)
  local pipe = io.popen(STARSHIP .. " prompt" .. target.flag .. arguments .. " 2>nul")
  if not pipe then
    return ""
  end
  local prompt = pipe:read("*a") or ""
  pipe:close()
  return prompt
end

local function batch_escape(value)
  -- Percent expansion is active even inside quotes in a batch file.
  return value:gsub("%%", "%%%%")
end

local function new_temp_file(prefix, extension)
  local file, name = os.createtmpfile(prefix, extension)
  if file then
    file:close()
  end
  return name
end

local function unlink(name)
  if name then
    os.unlink(name)
  end
end

local function cleanup(target)
  unlink(target.output)
  unlink(target.done)
  unlink(target.batch)
  target.output = nil
  target.done = nil
  target.batch = nil
  target.offset = 0
  target.buffer = ""
end

local function read_new_bytes(target)
  if not target.output then
    return ""
  end
  local file = io.open(target.output, "rb")
  if not file then
    return ""
  end
  file:seek("set", target.offset or 0)
  local bytes = file:read("*a") or ""
  target.offset = (target.offset or 0) + #bytes
  file:close()
  return bytes
end

local function take_frame(target)
  local first = target.buffer:find("\0", 1, true)
  if not first then
    return
  end
  local second = target.buffer:find("\0", first + 1, true)
  if not second then
    return
  end
  local third = target.buffer:find("\0", second + 1, true)
  if not third then
    return
  end

  local kind = target.buffer:sub(1, first - 1)
  local payload = target.buffer:sub(first + 1, second - 1)
  local auxiliary = target.buffer:sub(second + 1, third - 1)
  target.buffer = target.buffer:sub(third + 1)
  return kind, payload, auxiliary
end

local function consume(target)
  target.buffer = (target.buffer or "") .. read_new_bytes(target)
  local changed = false
  while true do
    local kind, payload, auxiliary = take_frame(target)
    if not kind then
      break
    elseif kind == "READY" then
      target.prompt = payload
      target.process = tonumber(auxiliary)
      target.ready = true
      changed = true
    elseif kind == "PATCH" then
      target.prompt = payload
      changed = true
    elseif kind == "COMPLETE" then
      target.timings = payload
      target.complete = true
    end
  end
  return changed
end

local function process_finished(target)
  if not target.done then
    return false
  end
  local file = io.open(target.done, "rb")
  if not file then
    return false
  end
  file:close()
  return true
end

local function launch(target, arguments)
  cleanup(target)
  target.output = new_temp_file("starship_" .. target.name, ".frames")
  target.done = new_temp_file("starship_" .. target.name, ".done")
  target.batch = new_temp_file("starship_" .. target.name, ".cmd")
  if not target.output or not target.done or not target.batch then
    cleanup(target)
    return false
  end

  -- The done file starts absent and is created by the wrapper only after the
  -- renderer closes stdout. This distinguishes a temporarily empty append-only
  -- stream from EOF without ever blocking Clink's main coroutine on a pipe.
  unlink(target.done)
  local batch = io.open(target.batch, "wb")
  if not batch then
    cleanup(target)
    return false
  end
  batch:write("@echo off\r\n")
  batch:write(
    batch_escape(STARSHIP),
    " stream", target.flag, arguments,
    " >\"", batch_escape(target.output), "\" 2>nul\r\n"
  )
  batch:write(">\"", batch_escape(target.done), "\" echo done\r\n")
  batch:close()

  target.offset = 0
  target.buffer = ""
  target.process = nil
  target.ready = false
  target.complete = false
  target.running = true

  local launch_command = 'start "" /b cmd.exe /d /q /c call "' .. target.batch .. '"'
  os.execute(launch_command)
  return true
end

local function stop(target)
  if not target.running then
    cleanup(target)
    return
  end

  -- READY is normally already consumed. If Enter wins the race by a few
  -- milliseconds, give the renderer a tiny bounded window to publish its PID
  -- so it cannot become an orphaned dynamic stream.
  if not target.process then
    for _ = 1, 10 do
      consume(target)
      if target.process or process_finished(target) then
        break
      end
      os.sleep(0.01)
    end
  end

  if target.process then
    os.execute(
      "taskkill /f /t /pid " .. target.process .. " >nul 2>nul"
    )
  end
  target.running = false
  target.process = nil
  cleanup(target)
end

local function stop_all()
  for _, target in ipairs(TARGETS) do
    stop(target)
  end
end

local function synchronous_prompts(session)
  for _, target in ipairs(TARGETS) do
    target.prompt = render(target, session.arguments)
  end
end

local function stream_prompts(session, background)
  if not background then
    synchronous_prompts(session)
    return true
  end

  local co = coroutine.running()
  clink.setcoroutinename(co, "starship prompt stream")
  clink.setcoroutineinterval(co, POLL_INTERVAL)

  local running = 0
  for _, target in ipairs(TARGETS) do
    if launch(target, session.arguments) then
      running = running + 1
    else
      target.prompt = render(target, session.arguments)
    end
  end

  if running < #TARGETS then
    clink.refilterprompt()
  end

  while current_session == session and session.active and running > 0 do
    local changed = false
    for _, target in ipairs(TARGETS) do
      if target.running then
        changed = consume(target) or changed
        if process_finished(target) then
          changed = consume(target) or changed
          if not target.complete then
            target.prompt = render(target, session.arguments)
            changed = true
          end
          target.running = false
          target.process = nil
          cleanup(target)
          running = running - 1
        end
      end
    end
    if changed and current_session == session and session.active then
      clink.refilterprompt()
    end
    if running > 0 then
      coroutine.yield()
    end
  end
  return true
end

clink.onbeginedit(function()
  local end_time = os.clock()
  if not is_line_empty then
    command_duration = end_time - start_time
  end

  generation = generation + 1
  current_session = {
    active = true,
    arguments = shell_arguments(),
    generation = generation,
    preprompt_called = false,
  }
  for _, target in ipairs(TARGETS) do
    target.prompt = ""
  end
end)

clink.onendedit(function(curr_line)
  if current_session then
    current_session.active = false
  end
  stop_all()

  if starship_precmd_user_func ~= nil then
    starship_precmd_user_func(curr_line)
  end
  start_time = os.clock()
  is_line_empty = #curr_line:match("^%s*(.-)%s*$") == 0
end)

function starship_prompt:filter(prompt)
  local session = current_session
  if not session then
    return render(TARGETS[1], shell_arguments())
  end

  if not session.preprompt_called then
    session.preprompt_called = true
    if starship_preprompt_user_func ~= nil then
      starship_preprompt_user_func(prompt)
    end
  end

  if STREAMING_CLINK then
    clink.promptcoroutine(function(background)
      return stream_prompts(session, background)
    end)
  else
    TARGETS[1].prompt = render(TARGETS[1], session.arguments)
  end
  return TARGETS[1].prompt
end

function starship_prompt:rightfilter(prompt)
  if not STREAMING_CLINK then
    local arguments = current_session and current_session.arguments or shell_arguments()
    TARGETS[2].prompt = render(TARGETS[2], arguments)
  end
  return TARGETS[2].prompt
end

if starship_transient_prompt_func ~= nil then
  function starship_prompt:transientfilter(prompt)
    return starship_transient_prompt_func(prompt)
  end
end

if starship_transient_rprompt_func ~= nil then
  function starship_prompt:transientrightfilter(prompt)
    return starship_transient_rprompt_func(prompt)
  end
end

local characterset = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
local randomkey = ""
math.randomseed(os.time())
for _ = 1, 16 do
  local index = math.random(#characterset)
  randomkey = randomkey .. characterset:sub(index, index)
end

os.setenv("STARSHIP_SHELL", "cmd")
os.setenv("STARSHIP_SESSION_KEY", randomkey)
