call variables.cmd

call variables.cmd
call bundle.bat
call encode-csv-strings.bat

rmdir "%modpath%\release" /s /q
mkdir "%modpath%\release"

mkdir "%modpath%\release\mods\%modname%\content\scripts\"
rmdir "%modpath%\release\mods\%modName%\content\" /s /q

@REM pack everything into a single a file and put the file in the release folder
mkdir "%modpath%\release\mods\%modname%\content\scripts\local\
XCOPY "%modPath%\src\vanilla\" "%modpath%\release\mods\%modname%\content\scripts\" /e /s /y
> "%modpath%\release\mods\%modname%\content\scripts\local\ga_scripts.min.ws" (for /r "%modpath%\src\local" %%F in (*.ws) do @type "%%F")

XCOPY "%modpath%\wolvenkit-project\packed\" "%modpath%\release\dlc\dlc%modname%\" /e /s /y
XCOPY "%modpath%\strings" "%modpath%\release\mods\%modName%\content\" /e /s /y

mkdir "%modpath%\release\bin\config\r4game\user_config_matrix\pc\"
copy "%modpath%\mod-menu.xml" "%modpath%\release\bin\config\r4game\user_config_matrix\pc\%modname%.xml" /y

REM Shared utilities
XCOPY "%modpath%\sharedutils\mod_sharedutils_dialogChoices\" "%modpath%\release\mods\mod_sharedutils_dialogChoices\" /e /s /y
XCOPY "%modpath%\sharedutils\mod_sharedutils_npcInteraction\" "%modpath%\release\mods\mod_sharedutils_npcInteraction\" /e /s /y
XCOPY "%modpath%\sharedutils\mod_sharedutils_helpers\" "%modpath%\release\mods\mod_sharedutils_helpers\" /e /s /y