@context(
  define("mod.aGwentEmpire")
  file("game/gui/hud/modules/hudModuleDialog.ws")
  at(class CR4HudModuleDialog)
)

@insert(
  at(event  OnDialogOptionAccepted)
  below(var progress : float)
)
// modAGwentEmpire - BEGIN
var overridden_index: int;
// modAGwentEmpire - END

@insert(
  at(event  OnDialogOptionAccepted)
  below(var progress : float) 
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
  at(function SendDialogChoicesToUI)
  above(lastSetChoices = choices)
)
// modAGwentEmpire - BEGIN
choices = GA_injectGwentDialogOption(choices, choiceFlashArray, this);
// modAGwentEmpire - END