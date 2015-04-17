obj = {
  key: value
  key: value
}

obj = {
  key: value,
  key: value
}

obj =
  key: value
  key: value

obj =
  key: value,
  key: value

/* all compiled to
var obj;

obj = {
  key: value,
  key: value
};
*/
