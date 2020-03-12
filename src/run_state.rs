use specs::Entity;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
  AwaitingInput,
  PreRun,
  PlayerTurn,
  MonsterTurn,
  InventoryMenu,
  DropItemMenu,
  ShowTargeting { range: i32, item: Entity },
  MainMenu { highlighted: usize },
  ExitGameMenu { highlighted: usize },
  DeathScreen,
  IntroScreen,
  FailureScreen,
  SuccessScreen,
  CreditsScreen,
}
