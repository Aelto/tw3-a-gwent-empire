

function GA_identifierToInt(identifier: string): int {
  var character: string;
  var segment: string;
  var output: int;

  segment = identifier;

  while (StrLen(segment) > 0) {
    character = StrLeft(segment, 1);

    output += GA_alphaToInt(character);

    // this takes the rest of the characters in the segment
    // everything except the first character.
    segment = StrMid(segment, 1);
  }

  return output;
}

function GA_alphaToInt(letter: string): int {
  var charcode: int;

  // this is just in case the received string is
  // longer than one character.
  letter = StrLeft(letter, 1);

  switch (letter) {
    case "a":
      charcode = 97;
      break;
    case "b":
      charcode = 98;
      break;
    case "c":
      charcode = 99;
      break;
    case "d":
      charcode = 100;
      break;
    case "e":
      charcode = 101;
      break;
    case "f":
      charcode = 102;
      break;
    case "g":
      charcode = 103;
      break;
    case "h":
      charcode = 104;
      break;
    case "i":
      charcode = 105;
      break;
    case "j":
      charcode = 106;
      break;
    case "k":
      charcode = 107;
      break;
    case "l":
      charcode = 108;
      break;
    case "m":
      charcode = 109;
      break;
    case "n":
      charcode = 110;
      break;
    case "o":
      charcode = 111;
      break;
    case "p":
      charcode = 112;
      break;
    case "q":
      charcode = 113;
      break;
    case "r":
      charcode = 114;
      break;
    case "s":
      charcode = 115;
      break;
    case "t":
      charcode = 116;
      break;
    case "u":
      charcode = 117;
      break;
    case "v":
      charcode = 118;
      break;
    case "w":
      charcode = 119;
      break;
    case "x":
      charcode = 120;
      break;
    case "y":
      charcode = 121;
      break;
    case "z":
      charcode = 122;
      break;
    case " ":
      charcode = 32;
      break;
    case "-":
      charcode = 45;
      break;
    case "_":
      charcode = 95;
      break;
    
    // if nothing matched, in a last attempt we try
    // to parseInt the string.
    default:
      charcode = StringToInt(letter);
      break;
  }

  return charcode;
}