a = [];
[].push.call(a, 0);
[].push.call(a, 1);
a;  //[0, 1]

b = {};
[].push.call(b, 0);
[].push.call(b, 1);
b;  //{"0":0, "1":1, "length":2}

c = {1:"a"}
[].push.call(c, 0);
c;  //{"0":0, "1":"a", "length":1}
[].push.call(c, 1);
c;  //{"0":0, "1":1, "length":2}
[].push.call(c, 2);
c;  //{"0":0, "1":1, "2":2, "length":3}

d = {"length":10}
[].push.call(d, 0);
d;  //{"10":1, "length":11}

