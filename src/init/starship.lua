if (clink.version_encoded or 0) < 10020030 then
  error("Starship requires a newer version of Clink; please upgrade to Clink v1.2.30 or later.")
end

local starship_prompt = clink.promptfilter(5)

start_time = os.clock()
end_time = 0
curr_duration = 0
is_line_empty = true

clink.onbeginedit(function ()
  end_time = os.clock()
  if not is_line_empty then
    curr_duration = end_time - start_time
  end
end)

clink.onendedit(function (curr_line)
  if starship_precmd_user_func ~= nil then
    starship_precmd_user_func(curr_line)
  end
  start_time = os.clock()
  if string.len(string.gsub(curr_line, '^%s*(.-)%s*$', '%1')) == 0 then
    is_line_empty = true
  else
    is_line_empty = false
  end
end)

function starship_prompt:filter(prompt)
  if starship_preprompt_user_func ~= nil then
    starship_preprompt_user_func(prompt)
  end
  return io.popen([[::STARSHIP::]].." prompt"
    .." --status="..os.geterrorlevel()
    .." --cmd-duration="..math.floor(curr_duration*1000)
    .." --terminal-width="..console.getwidth()
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end

function starship_prompt:rightfilter(prompt)
  return io.popen([[::STARSHIP::]].." prompt --right"
    .." --status="..os.geterrorlevel()
    .." --cmd-duration="..math.floor(curr_duration*1000)
    .." --terminal-width="..console.getwidth()
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
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
for i = 1, 16 do
  local rand = math.random(#characterset)
  randomkey = randomkey..string.sub(characterset, rand, rand)
end

os.setenv('STARSHIP_SHELL', 'cmd')
os.setenv('STARSHIP_SESSION_KEY', randomkey)
