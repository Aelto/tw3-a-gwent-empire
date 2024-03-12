@echo off
call variables.cmd

tw3-cahirp build  --game "%gamePath%" --without-mods --recipes "%modpath%/src/%modname%/cahirp" --out "%modpath%/merges" --clean

XCOPY "%modpath%\merges\" "%modpath%\src\%modname%\content\scripts" /e /s /y
rmdir "%modpath%\merges\" /s /q