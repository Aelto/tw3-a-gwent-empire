
class GA_VanillaCardsDictionnary extends GA_CardsDictionnary {
  public function init() {

  }

  private function initNorthernRealm() {
    var faction: GA_FactionCardDictionnary;

    faction = GA_FactionCardDictionnary();
    
    faction.hero_cards.PushBack(GA_CardEntry(
      ImlerithHero, // index
      1, // maximum
      1, // minimum
      GA_DeckPoints(10),
      GA_MatchDifficulty(100)
    ));
  }

  public function populate(difficulty: GA_MatchDifficulty, points: GA_DeckPoints) {

  }
}

enum GA_VanillaCardsIndex {
  // northern kingdom
  GA_VCI_Vernon = 100,
  GA_VCI_Natalis = 101,
  GA_VCI_Esterad = 102,
  GA_VCI_Philipa = 103,
  GA_VCI_Thaler = 105,
  GA_VCI_Siegfried- = 107,
  GA_VCI_Dijsktra = 109,
  GA_VCI_Stennis = 116,
  GA_VCI_Trebuchet = 121,
  GA_VCI_PoorInfantry = 125,
  GA_VCI_PoorInfantry2 = 126,
  GA_VCI_PoorInfantry3 = 127,
  GA_VCI_Crinfrid = 130,
  GA_VCI_Catapult = 140,
  GA_VCI_Ballista = 146,
  GA_VCI_Kaedwen = 150,
  GA_VCI_Kaedwen = 151,
  GA_VCI_BlueStripes = 160,
  GA_VCI_SiegeTower = 170,
  GA_VCI_DunBannerMedic = 175,

  // nilfgaardian
  GA_VCI_Letho = 200,
  GA_VCI_Menno = 201,
  GA_VCI_Moorvran = 202,
  GA_VCI_Tibor = 203,
  GA_VCI_Albrich = 205,
  GA_VCI_Assire = 206,
  GA_VCI_Cynthia = 207,
  GA_VCI_Fringilla = 208,
  GA_VCI_Mortheisen = 209,
  GA_VCI_Rainfarn = 210,
  GA_VCI_Renual = 211,
  GA_VCI_Rotten = 212,
  GA_VCI_Shilard = 213,
  GA_VCI_Stefan = 214,
  GA_VCI_Sweers = 215,
  GA_VCI_Vanhemar = 217,
  GA_VCI_Vattier = 218,
  GA_VCI_Vreemde = 219,
  GA_VCI_Cahir = 220,
  GA_VCI_Puttkammer = 221,
  GA_VCI_ArcherSupport = 230,
  GA_VCI_ArcherSupport2 = 231,
  GA_VCI_BlackArcher = 235,
  GA_VCI_BlackArcher2 = 236,
  GA_VCI_HeavyZerri = 240,
  GA_VCI_Zerri = 241,
  GA_VCI_ImperaBrigade = 245,
  GA_VCI_CombatEngineer = 255,
  GA_VCI_Nausicaa = 250,
  GA_VCI_YoungEmissary = 260,
  GA_VCI_YoungEmissary = 261,
  GA_VCI_SiegeSupport = 265,

  // Scoiatael
  GA_VCI_Eithne = 300,
  GA_VCI_Saskia = 301,
  GA_VCI_Isengrim = 302;,
  GA_VCI_Iorveth = 303,
  GA_VCI_Milva = 306,
  GA_VCI_Dennis = 305,
  GA_VCI_Ida = 307,
  GA_VCI_Filavandrel = 308,
  GA_VCI_Yaevinn = 309,
  GA_VCI_Toruviel = 310,
  GA_VCI_Riordain = 311,
  GA_VCI_Ciaran = 312,
  GA_VCI_Barclay = 313,
  GA_VCI_HavekarSupport = 320,
  GA_VCI_HavekarSupport2 = 321,
  GA_VCI_HavekarSupport3 = 322,
  GA_VCI_VriheddBrigade = 325,
  GA_VCI_VriheddBrigade2 = 326,
  GA_VCI_DolInfantry = 330,
  GA_VCI_DolInfantry2 = 331,
  GA_VCI_DolInfantry3 = 332,
  GA_VCI_DolDwarf = 335,
  GA_VCI_DolDwarf2 = 336,
  GA_VCI_DolDwarf3 = 337,
  GA_VCI_Mahakam = 340,
  GA_VCI_Mahakam2 = 341,
  GA_VCI_Mahakam3 = 342,
  GA_VCI_Mahakam4 = 343,
  GA_VCI_Mahakam5 = 344,
  GA_VCI_ElfSkirmisher = 350,
  GA_VCI_ElfSkirmisher2 = 351,
  GA_VCI_ElfSkirmisher3 = 352,
  GA_VCI_VriheddCadet = 355,
  GA_VCI_DolArcher = 360,
  GA_VCI_HavekarNurse = 365,
  GA_VCI_HavekarNurse2 = 366,
  GA_VCI_HavekarNurse3 = 367,
  GA_VCI_Schirru = 368,

  // Monster
  GA_VCI_Draug = 400,
  GA_VCI_Kayran = 401,
  GA_VCI_Imlerith = 402,
  GA_VCI_Leshen = 403,
  GA_VCI_Forktail = 405,
  GA_VCI_EarthElemental = 407,
  GA_VCI_Fiend = 410,
  GA_VCI_PlagueMaiden = 413,
  GA_VCI_Griffin = 415,
  GA_VCI_Werewolf = 417,
  GA_VCI_Botchling = 420,
  GA_VCI_Frightener = 423,
  GA_VCI_Ice_giant = 425,
  GA_VCI_Endrega = 427,
  GA_VCI_Harpy = 430,
  GA_VCI_Cockatrice = 433,
  GA_VCI_Gargoyle = 435,
  GA_VCI_Celaeno_harpy = 437,
  GA_VCI_Grave_hag = 44,
  GA_VCI_FireElemental = 44,
  GA_VCI_Fogling = 445,
  GA_VCI_Wyvern = 447,
  GA_VCI_ArachasBehemoth = 450,
  GA_VCI_Arachas = 451,
  GA_VCI_Arachas2 = 452,
  GA_VCI_Arachas3 = 453,
  GA_VCI_Nekker = 455,
  GA_VCI_Nekker2 = 456,
  GA_VCI_Nekker3 = 457,
  GA_VCI_Ekkima = 460,
  GA_VCI_Fleder = 461,
  GA_VCI_Garkain = 462,
  GA_VCI_Bruxa = 463,
  GA_VCI_Katakan = 464,
  GA_VCI_Ghoul = 470,
  GA_VCI_Ghoul2 = 471,
  GA_VCI_Ghoul3 = 472,
  GA_VCI_CroneBrewess = 47,
  GA_VCI_Toad = 47,

  // Skellige
  GA_VCI_CrachAnCraite = 500,
  GA_VCI_Hjalmar = 501,
  GA_VCI_Cerys = 502,
  GA_VCI_Ermion = 503,
  GA_VCI_Draig = 504,
  GA_VCI_HolgerBlackhand = 505,
  GA_VCI_MadmanLugos = 506,
  GA_VCI_DonarAnHindar = 507,
  GA_VCI_Udalryk = 508,
  GA_VCI_BirnaBran = 509,
  GA_VCI_BlueboyLugos = 510,
  GA_VCI_Svanrige = 511,
  GA_VCI_Olaf = 512,
  GA_VCI_Berserker = 513,
  GA_VCI_Young_berserker = 515,
  GA_VCI_ClanAnCraiteWarrior = 517,
  GA_VCI_ClanTordarrochArmorsmith = 518,
  GA_VCI_Clan_heymaey_skald = 519,
  GA_VCI_LightDrakkar = 520,
  GA_VCI_WarDrakkar = 521,
  GA_VCI_ClanBrokvarArcher = 522,
  GA_VCI_ClanDrummondShieldmaiden = 523,
  GA_VCI_ClanDrummondShieldmaiden2 = 526,
  GA_VCI_ClanDrummondShieldmaiden3 = 527,
  GA_VCI_ClanDimunPirate = 524,
  GA_VCI_Cock = 525,

  // Neutral
  GA_VCI_Geralt = 7,
  GA_VCI_Vesemir = 8, 
  GA_VCI_Yennefer = 9, 
  GA_VCI_Ciri = 10,
  GA_VCI_Triss = 11,
  GA_VCI_Dandelion = 12,
  GA_VCI_Zoltan = 13,
  GA_VCI_Emiel = 14,
  GA_VCI_Villen = 15,
  GA_VCI_Avallach = 16,
  GA_VCI_Olgierd = 17,
  GA_VCI_Mrmirror = 18,
  GA_VCI_MrmirrorFoglet = 19,
  GA_VCI_Cow = 20,
  GA_VCI_LadyOfTheLake = 24,
  GA_VCI_Visenna = 25,

  // Special
  GA_VCI_Dummy = 0, 
  GA_VCI_Horn = 1, 
  GA_VCI_Scorch = 2, 
  GA_VCI_Frost = 3, 
  GA_VCI_Fog = 4, 
  GA_VCI_Rain = 5, 
  GA_VCI_ClearSky = 6,
  GA_VCI_Mushroom = 22,
  GA_VCI_SkelligeStorm = 23,

  // Leaders
  GA_VCI_foltest_bronze = 1002,
  GA_VCI_foltest_silver = 1003,
  GA_VCI_foltest_gold = 1004,
  GA_VCI_foltest_platinium = 1005,
  GA_VCI_emhyr_bronze = 2002,
  GA_VCI_emhyr_silver = 2003,
  GA_VCI_emhyr_gold = 2004,
  GA_VCI_emhyr_platinium = 2005,
  GA_VCI_francesca_bronze = 3002,
  GA_VCI_francesca_silver = 3003,
  GA_VCI_francesca_gold = 3004,
  GA_VCI_francesca_platinium = 3005,
  GA_VCI_eredin_bronze = 4002,
  GA_VCI_eredin_silver = 4003,
  GA_VCI_eredin_gold = 4004,
  GA_VCI_eredin_platinium = 4005,
  GA_VCI_king_bran_bronze = 5001,
  GA_VCI_king_bran_copper = 5002,
}