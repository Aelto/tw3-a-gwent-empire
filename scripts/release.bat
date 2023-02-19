call variables.cmd

rmdir "%modpath%\release" /s /q
mkdir "%modpath%\release"

:: here we compile the cahirc scripts, then construct the final mods by combining
:: the vanilla edits of the mods and their local scripts:
call compile
@REM call bundle.bat

set modname=modGwentAddict
XCOPY "%modpath%\src\%modname%\" "%modpath%\release\mods\%modname%\" /e /s /y
rmdir "%modpath%\release\mods\%modname%\content\scripts\local\" /s /q
XCOPY "%modpath%\dist\%modname%\" "%modpath%\release\mods\%modname%\" /e /s /y

:: move the strings
XCOPY "%modpath%\strings" "%modpath%\release\mods\%modName%\content\" /e /s /y

echo copy the DLC
mkdir "%modspot%\release\dlc"
XCOPY "%modpath%\wolvenkit-project\packed\" "%modpath%\release\dlc\dlc%modname%\" /e /s /y

echo copy the sharedutils dependencies
set modname=mod_sharedutils_npcInteraction
XCOPY "%modpath%\sharedutils\%modname%\" "%modpath%\release\mods\%modname%\" /e /s /y
set modname=mod_sharedutils_storage
XCOPY "%modpath%\sharedutils\%modname%\" "%modpath%\release\mods\%modname%\" /e /s /y
set modname=mod_sharedutils_helpers
XCOPY "%modpath%\sharedutils\%modname%\" "%modpath%\release\mods\%modname%\" /e /s /y

:: don't need a menu at the moment
@REM mkdir "%modpath%\release\bin\config\r4game\user_config_matrix\pc\"
@REM copy "%modpath%\mod-menu.xml" "%modpath%\release\bin\config\r4game\user_config_matrix\pc\%modname%.xml" /y
