@context(
  note("hudModuleDialog changes")
  define("mod.aGwentEmpire")
  file("game/gui/hud/modules/hudModuleDialog.ws")
  at(class CR4HudModuleDialog)
)

@insert(
  note("hook dialog accept event for override")
  note("add dialog injecting variables")
  at(OnDialogOptionAccepted)
  below(var progress : float)
)
// modAGwentEmpire - BEGIN
var overridden_index: int;
// modAGwentEmpire - END

@insert(
  note("hook dialog accept event for override")
  note("override dialog index")
  at(OnDialogOptionAccepted)
  below(var progress)
  below(LogChannel)
)
// modAGwentEmpire - BEGIN
overridden_index = GA_onDialogOptionAccepted(acceptedChoice, lastSetChoices);
if (overridden_index >= 0) {
  index = overridden_index;
  acceptedChoice = lastSetChoices[index];
}
// modAGwentEmpire - END

@insert(
  note("add dialog injection")
  at(function SendDialogChoicesToUI)
  above(lastSetChoices = choices)
)
// modAGwentEmpire - BEGIN
choices = GA_injectGwentDialogOption(choices, choiceFlashArray, this);
// modAGwentEmpire - END