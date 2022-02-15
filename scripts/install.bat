@echo off

call variables.cmd

rem install scripts
rmdir "%gamePath%\mods\%modName%\content\scripts" /s /q
@REM first the vanilla scripts
XCOPY "%modPath%\src\vanilla\" "%gamePath%\mods\%modName%\content\scripts\" /e /s /y
@REM then the local files
XCOPY "%modPath%\src\local\" "%gamePath%\mods\%modName%\content\scripts\local\" /e /s /y
XCOPY "%modPath%\strings" "%gamepath%\mods\%modname%\content\" /e /s /y
copy "%modPath%\mod-menu.xml" "%gamePath%\bin\config\r4game\user_config_matrix\pc\%modname%.xml" /y

rem install sharedutils
rmdir "%gamePath%\mods\mod_sharedutils_npcInteraction" /s /q
XCOPY "%modPath%\sharedutils\mod_sharedutils_npcInteraction" "%gamePath%\mods\mod_sharedutils_npcInteraction\" /e /s /y

rmdir "%gamePath%\mods\mod_sharedutils_dialogChoices" /s /q
XCOPY "%modPath%\sharedutils\mod_sharedutils_dialogChoices" "%gamePath%\mods\mod_sharedutils_dialogChoices\" /e /s /y

rmdir "%gamePath%\mods\mod_sharedutils_helpers" /s /q
XCOPY "%modPath%\sharedutils\mod_sharedutils_helpers" "%gamePath%\mods\mod_sharedutils_helpers\" /e /s /y

if "%1"=="-dlc" (
  echo "copying DLC"
  rmdir "%gamePath%\dlc\dlc%modName%" /s /q
  xcopy "%modPath%\release\dlc" "%gamepath%\dlc" /e /s /y
)
