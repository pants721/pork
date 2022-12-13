#SingleInstance, Force
SendMode Input
SetWorkingDir, %A_ScriptDir%

P:: 
    Run %A_ScriptDir%\target\release\evil-pork.exe run
    ; WinActivate, Fullscreen Projector (Preview)
J:: Reload