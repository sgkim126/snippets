import random
import time
import unittest


def russian(a, b): 
    result = 0
    while a != 0:
        if a % 2 == 1:
            result += b
        a >>= 1
        b <<= 1
    return result

def mul(a, b):
    return a * b

def rand():
    return int(random.uniform(100, 100000))

if __name__ == '__main__':
    COUNT = 10000
    russian_times = []
    normal_times = []
    for _ in range(10000):
        a = rand()
        b = rand()

        start_russian = time.time()
        russian_result = russian(a, b)
        elapsed_for_russian = time.time() - start_russian

        start_normal = time.time()
        normal_result = mul(a, b)
        elapsed_for_normal = time.time() - start_normal

        assert russian_result == normal_result, "%s * %s = %d vs %d" % (a, b, russian_result, normal_result)
        russian_times.append(elapsed_for_russian)
        normal_times.append(elapsed_for_normal)

    russian_times.sort()
    print("russian\n avg: %s ms\n min: %s ms\n q1: %s ms\n a2: %s ms\n q3:%s ms\n max: %s ms\n" % (
        sum(russian_times) / COUNT * 1000,
        russian_times[0] * 1000,
        russian_times[int(COUNT / 4)] * 1000,
        russian_times[int(COUNT / 2)] * 1000,
        russian_times[int(COUNT * 3 / 4)] * 1000,
        russian_times[-1] * 1000))

    normal_times.sort()
    print("normal\n avg: %s ms\n min: %s ms\n q1: %s ms\n a2: %s ms\n q3:%s ms\n max: %s ms\n" % (
        sum(normal_times) / COUNT * 1000,
        normal_times[0] * 1000,
        normal_times[int(COUNT / 4)] * 1000,
        normal_times[int(COUNT / 2)] * 1000,
        normal_times[int(COUNT * 3 / 4)] * 1000,
        normal_times[-1] * 1000))
