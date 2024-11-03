
@addField(CR4Player) 
saved var a_gwent_empire: GA_Storage;

@addField(CR4Player)
saved var GA_cards_dictionary: GA_CardsDictionary

@addMethod(CR4Player)
public function getAGwentEmpire(): GA_Storage {
  if (!this.a_gwent_empire) {
    this.a_gwent_empire = new GA_Storage in this;
  }

  return this.a_gwent_empire;
}

@addMethod(CR4Player)
public function setAGwentEmpire(storage: GA_Storage) {
  this.a_gwent_empire = storage;
}

@addMethod(CR4Player)
public function AGE_setCardsDictionnary(dictionnary: GA_CardsDictionary) {
  this.GA_cards_dictionary = dictionnary;
}

@wrapMethod(CR4Player)
function OnSpawned(spawnData: SEntitySpawnData) {
  var age: GA_Storage;

  wrappedMethod(spawnData);

  age = this.getAGwentEmpire();
}