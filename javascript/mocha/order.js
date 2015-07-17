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

  describe('Mocha order nested', function () {
    before(function () {
      console.log('nested before');
    });

    after(function () {
      console.log('nested after');
    });

    beforeEach(function () {
      console.log('nested beforeEach');
    });
    afterEach(function () {
      console.log('nested afterEach');
    });

    it('test1', function () {
      console.log('nested test1');
    });
    it('test2', function () {
      console.log('nested test2');
    });
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
//     Mocha order nested
// nested before
// beforeEach
// nested beforeEach
// nested test1
//       ✓ test1
// nested afterEach
// afterEach
// beforeEach
// nested beforeEach
// nested test2
//       ✓ test2
// nested afterEach
// afterEach
// nested after
// after
