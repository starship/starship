if (clink.version_encoded or 0) < 10020030 then
  error("Starship requires a newer version of Clink; please upgrade to Clink v1.2.30 or later.")
end

local STARSHIP = [[::STARSHIP::]]
-- Clink's documented async door, io.popenyield, hands back a handle only once
-- the child has finished, so it can deliver a prompt but never refine one.
-- popenyield_internal returns the handle immediately, which is the whole reason
-- frames can stream — but it is marked "internal use only" in Clink's source,
-- so it is probed rather than assumed, and its absence costs only streaming.
local STREAM = (clink.version_encoded or 0) >= 10080000 and io.popenyield_internal ~= nil

-- One renderer draws both sides, so one process feeds two prompt strings. `left`
-- and `right` hold only what their filters hand back; `renderer` is that one
-- process — its temp file, its yieldguard, the cursor walking its frames, and
-- the timings carried forward to prime the next prompt. A single prompt filter
-- runs a single prompt coroutine (Clink caches one per filter per input line),
-- so this one coroutine reads this one stream and refills both sides.
local left = { flag = "", prompt = "" }
local right = { flag = " --right", prompt = "" }
local renderer = { timings = "" }
local starship_prompt = clink.promptfilter(5)
local session, command_start, duration, empty = nil, os.clock(), 0, true

local function arguments()
  return " --status=" .. os.geterrorlevel()
    .. " --cmd-duration=" .. math.floor(duration * 1000)
    .. " --terminal-width=" .. console.getwidth()
    .. " --keymap=" .. (rl.getvariable("keymap") or "emacs-standard")
end

-- `async` says this is running inside the prompt coroutine, where a plain
-- io.popen would block Clink's whole idle scheduler — and with it the line
-- editor — for as long as the render takes. io.popenyield yields instead.
local function render(side, extra, async)
  local open = (async and io.popenyield) or io.popen
  local pipe = open(STARSHIP .. " prompt" .. side.flag .. extra .. " 2>nul")
  if not pipe then
    return ""
  end
  local text = pipe:read("*a") or ""
  pipe:close()
  return text
end

-- Draws both sides synchronously, the fallback whenever a stream cannot serve
-- them: no streaming build, a renderer that would not launch, or one whose pipe
-- closed before it finished.
local function render_both(extra, async)
  left.prompt = render(left, extra, async)
  right.prompt = render(right, extra, async)
end

-- Compact frames are KEYWORD\0first\0second\0. Seek-before-read clears CRT EOF
-- so Clink's copy thread can append; never read a pipe from Lua (it would
-- freeze the UI). An empty read is not EOF — yieldguard:ready() is.
--
-- The one --both renderer sends both sides down this one pipe: READY and PATCH
-- carry the left prompt (READY's aux field is the renderer's pid, the only side
-- that announces it), RIGHT carries the right prompt, and a single COMPLETE
-- carries the merged timings once both sides have finished. HEARTBEAT only keeps
-- the pipe warm, so it falls through.
local function consume()
  if not renderer.file then
    return false
  end
  renderer.file:seek("set", renderer.offset)
  local chunk = renderer.file:read("*a") or ""
  if #chunk > 0 then
    -- Lua strings are immutable, so the buffer is rebuilt once per read, with
    -- whatever has already been parsed dropped on the way — never once per
    -- frame. A cursor walks the frames in place; plain `find` scans for the
    -- terminator without the backtracking a `.-` pattern would spend.
    renderer.buffer = (renderer.cursor > 1 and renderer.buffer:sub(renderer.cursor) or renderer.buffer) .. chunk
    renderer.cursor = 1
    renderer.offset = renderer.offset + #chunk
  end
  local changed = false
  while true do
    local first = renderer.buffer:find("\0", renderer.cursor, true)
    local second = first and renderer.buffer:find("\0", first + 1, true)
    local third = second and renderer.buffer:find("\0", second + 1, true)
    if not third then
      break
    end
    local kind = renderer.buffer:sub(renderer.cursor, first - 1)
    local payload = renderer.buffer:sub(first + 1, second - 1)
    local aux = renderer.buffer:sub(second + 1, third - 1)
    renderer.cursor = third + 1
    if kind == "READY" then
      left.prompt, renderer.pid, changed = payload, tonumber(aux), true
    elseif kind == "PATCH" then
      left.prompt, changed = payload, true
    elseif kind == "RIGHT" then
      right.prompt, changed = payload, true
    elseif kind == "COMPLETE" then
      renderer.timings, renderer.ok = payload, true
    end
  end
  return changed
end

local function stop()
  if not renderer.file then
    return
  end
  -- Clink has no kill of any kind, and closing the handle cannot stand in for
  -- one: popenyield hands Lua a temp file that a copy thread fills, so the
  -- renderer's real pipe stays open and its writes keep succeeding. taskkill
  -- is the only door, and it costs two Windows process spawns.
  --
  -- So only knock on it for a renderer that is still alive. A ready yieldguard
  -- means the child already closed stdout and exited, which is every prompt
  -- that finished rendering before Enter — the common case pays nothing, and
  -- only a stream cut off mid-render waits for a PID it can kill.
  if not renderer.yieldguard:ready() then
    -- Enter can beat READY, and the PID arrives with it.
    for _ = 1, 10 do
      if renderer.pid or renderer.yieldguard:ready() then
        break
      end
      os.sleep(0.01)
      consume()
    end
    if renderer.pid and not renderer.yieldguard:ready() then
      os.execute("taskkill /f /t /pid " .. renderer.pid .. " >nul 2>nul")
    end
  end
  renderer.file:close()
  renderer.file, renderer.yieldguard, renderer.pid = nil
end

-- Public io.popenyield waits until complete output is ready, so it cannot stream
-- PATCH. The internal API returns a readable handle immediately and a yieldguard
-- that becomes ready when stdout closes. `--both` draws both sides from this one
-- process, so where Starship once spawned a renderer per side it now spawns one.
local function launch(extra)
  stop()
  if not io.popenyield_internal then
    return false
  end
  local file, yieldguard = io.popenyield_internal(
    STARSHIP .. " stream --both" .. extra
      .. ' "--timings=' .. (renderer.timings or ""):gsub('"', '""') .. '" 2>nul',
    "rb"
  )
  if not file then
    return false
  end
  renderer.file, renderer.yieldguard = file, yieldguard
  renderer.offset, renderer.buffer, renderer.cursor = 0, "", 1
  renderer.pid, renderer.ok = nil, false
  return true
end

local function stream(active_session, background)
  if not background then
    render_both(active_session.arguments)
    return true
  end

  clink.setcoroutineinterval(coroutine.running(), 0.02)

  if not launch(active_session.arguments) then
    -- No stream to adopt: draw both sides the slow way and show them at once.
    render_both(active_session.arguments, true)
    clink.refilterprompt()
    return true
  end

  while session == active_session and active_session.active and renderer.file do
    local changed = consume()
    if renderer.yieldguard:ready() then
      changed = consume() or changed
      -- The renderer exits only after the merged COMPLETE, so a pipe that closed
      -- without one was cut off mid-render; redraw both sides synchronously.
      if not renderer.ok then
        render_both(active_session.arguments, true)
        changed = true
      end
      renderer.file:close()
      renderer.file, renderer.yieldguard, renderer.pid = nil
    end
    if changed and session == active_session and active_session.active then
      clink.refilterprompt()
    end
    if renderer.file then
      coroutine.yield()
    end
  end
  return true
end

clink.onbeginedit(function()
  local now = os.clock()
  if not empty then
    duration = now - command_start
  end
  session = { active = true, arguments = arguments(), preprompt = false }
  left.prompt, right.prompt = "", ""
end)

clink.onendedit(function(line)
  if session then
    session.active = false
  end
  stop()
  if starship_precmd_user_func then
    starship_precmd_user_func(line)
  end
  command_start = os.clock()
  empty = #line:match("^%s*(.-)%s*$") == 0
end)

function starship_prompt:filter(prompt)
  if not session then
    return render(left, arguments())
  end
  if not session.preprompt then
    session.preprompt = true
    if starship_preprompt_user_func then
      starship_preprompt_user_func(prompt)
    end
  end
  if STREAM then
    clink.promptcoroutine(function(background)
      return stream(session, background)
    end)
  else
    left.prompt = render(left, session.arguments)
  end
  return left.prompt
end

function starship_prompt:rightfilter()
  if not STREAM then
    right.prompt = render(right, session and session.arguments or arguments())
  end
  return right.prompt
end

if starship_transient_prompt_func then
  function starship_prompt:transientfilter(prompt)
    return starship_transient_prompt_func(prompt)
  end
end
if starship_transient_rprompt_func then
  function starship_prompt:transientrightfilter(prompt)
    return starship_transient_rprompt_func(prompt)
  end
end

local alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
math.randomseed(os.time())
local key = {}
for i = 1, 16 do
  local n = math.random(#alphabet)
  key[i] = alphabet:sub(n, n)
end
os.setenv("STARSHIP_SHELL", "cmd")
os.setenv("STARSHIP_SESSION_KEY", table.concat(key))
