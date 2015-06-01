describe('Mocha order', function () {
  before(function () {
    console.log('before');
  });

  after(function () {
    console.log('after');
  });

  beforeEach(function () {
    console.log('beforeEach');
  });
  afterEach(function () {
    console.log('afterEach');
  });

  it('test1', function () {
    console.log('test1');
  });
  it('test2', function () {
    console.log('test2');
  });
});

//   Mocha order
// before
// beforeEach
// test1
//     ✓ test1
// afterEach
// beforeEach
// test2
//     ✓ test2
// afterEach
// after
// 
