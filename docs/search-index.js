var N = null,
  E = "",
  T = "t",
  U = "u",
  searchIndex = {};
var R = ["tostring", "string"];

searchIndex["cli_kit"] = {
  doc: E,
  i: [
    [0, "ansi", "cli_kit", E, N, N],
    [0, "color_codes", "cli_kit::ansi", E, N, N],
    [
      5,
      "red",
      "cli_kit::ansi::color_codes",
      "Take an item that implements ToString and return in red",
      N,
      [[[R[0]]], [R[1]]]
    ],
    [
      5,
      "red_bold",
      E,
      "Take an item that implements ToString and return in red bold",
      N,
      [[[R[0]]], [R[1]]]
    ],
    [
      5,
      "green",
      E,
      "Take an item that implements ToString and return in green",
      N,
      [[[R[0]]], [R[1]]]
    ],
    [
      5,
      "green_bold",
      E,
      "Take an item that implements ToString and return in green…",
      N,
      [[[R[0]]], [R[1]]]
    ]
  ],
  p: []
};
initSearch(searchIndex);
addSearchOptions(searchIndex);
