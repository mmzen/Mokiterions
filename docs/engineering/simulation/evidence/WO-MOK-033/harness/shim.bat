@echo off
python "%~dp0shim.py" %*
exit /b %ERRORLEVEL%
