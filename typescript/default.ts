function x() { console.log('called'); }

{
    const { a = x() } = { a: 1 };
}
{
    const { a = x() } = { };
}
