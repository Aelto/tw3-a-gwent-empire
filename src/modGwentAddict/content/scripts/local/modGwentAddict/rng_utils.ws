/**
 * a small utility function that allows us to
 * roll the given rng `n` times before finally
 * returning the value obtained from the last
 * roll.
 *
 * This is useful since most of the random
 * generation done for the mod is from a seeded
 * RNG. But if we were to get the first roll
 * everytime from a fresh RNG we would always
 * get the same number.
 *
 * So with this function we instead tell it to
 * get a specific roll number, which we'll hard
 * code but make sure every function uses a
 * somewhat unique roll number.
 */
function GA_rngRolls(roll: int, out rng: RandomNumberGenerator, max: float, min: float): float {
  while (i > 1) {
    rng.next();

    i -= 1;
  }

  return rng.nextRange(max, min);
}