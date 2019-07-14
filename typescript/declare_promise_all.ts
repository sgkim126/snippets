/// <reference lib="es2015" />

interface PromiseConstructor {
    all<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>(values: [T1 | PromiseLike<T1>, T2 | PromiseLike<T2>, T3 | PromiseLike<T3>, T4 | PromiseLike <T4>, T5 | PromiseLike<T5>, T6 | PromiseLike<T6>, T7 | PromiseLike<T7>, T8 | PromiseLike<T8>, T9 | PromiseLike<T9>, T10 | PromiseLike<T10>, T11 | PromiseLike<T11>]): Promise<[T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11]>;
}

const tuple: [
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number
] = [
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10
];

const p: Promise<[
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number
]> = Promise.all(tuple);
