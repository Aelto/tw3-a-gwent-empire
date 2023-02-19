
function GA_generateRandomDeck(manager: CR4GwintManager): SDeckDefinition {
  var general_data: GA_GeneralData;
  var deck: SDeckDefinition;

  manager.GA_generalData.current_match = GA_GeneralDataCurrentMatch(
    // identifier:
    GA_getDeckIdentifierFromNpc(manager.GA_generalData.opponent)
  );
  

  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]
  deck.cardIndices.PushBack(236); // Black Archer [10]

  deck.dynamicCardRequirements.PushBack(manager.diff5);
  deck.dynamicCards.PushBack(2); 
  deck.dynamicCardRequirements.PushBack(manager.diff7);
  deck.dynamicCards.PushBack(2); 
  deck.dynamicCardRequirements.PushBack(manager.diff9);
  deck.dynamicCards.PushBack(103); 
  deck.dynamicCardRequirements.PushBack(manager.diff11);
  deck.dynamicCards.PushBack(102); 
  deck.dynamicCardRequirements.PushBack(manager.diff11);
  deck.dynamicCards.PushBack(109); 
  deck.dynamicCardRequirements.PushBack(manager.diff14);
  deck.dynamicCards.PushBack(101); 
  deck.dynamicCardRequirements.PushBack(manager.diff14);
  deck.dynamicCards.PushBack(12);

  deck.specialCard = 100; 
  deck.leaderIndex = 1001;

  return deck;
}

/**
 * Generate a unique DeckIdentifier from the NPC, calling this function 
 * twice using the same NPC should result in the same identifier. calling
 * it with two different NPCs should result in two completely different
 * identifiers.
 */
function GA_getDeckIdentifierFromNpc(npc: CNewNpc): GA_DeckIdentifier {
  var monster_category: EMonsterCategory;
  var can_be_hit_by_fists: bool;
  var can_be_targeted: bool;
  var is_teleporting: bool;
  var monster_name: name;

  var identifier: string;

  theGame.GetMonsterParamsForActor(
    npc,
    monster_category,
    monster_name,
    is_teleporting,
    can_be_targeted,
    can_be_hit_by_fists
  );

  // we add dashes between all numbers and things to make it more
  // readable. It is not a problem since the function that transforms
  // identifiers to seeds is capable of translating all sorts of characters.
  //
  // Some of the segments may seem redundant, be the more data we have
  // about a NPC the better it is.
  idenfitier += monster_name + "-";
  identifier += npc.GetName() + "-";
  identifier += np.I_GetDisplayName() + "-";
  idenfitier += monster_category + "-";
  idenfitier += is_teleporting + "-";
  idenfitier += can_be_targeted + "-";
  idenfitier += can_be_hit_by_fists + "-";

  return GA_DeckIdentifier(identifier);
}



/**
 * return the amount of points given for the supplied seed.
 * Points are used when generating a deck of cards, each card
 * has a cost and the points determine how many cards can be
 * in the decks.
 */
function GA_getDeckPointsFromSeed(seed: int): GA_DeckPoints {
  var rng: RandomNumberGenerator;

  rng = (new RandomNumberGenerator in thePlayer).setSeed(seed.value)
    .useSeed(true);

  return GA_DeckPoints((int)GA_rngRolls(
    GA_CONSTANTS_ENUM_POINTS_FROM_SEED_ROLL,
    rng,
    100,
    10
  ));
}
