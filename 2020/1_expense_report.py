from os.path import abspath, dirname
from math import factorial as fac
from functools import reduce

INPUT_FILE = dirname(abspath(__file__)) + "/inputs/1-1.txt"


def get_report(filename: str) -> list:
    """gets expense report from a filename"""
    expense_report = open(filename).read().split("\n")
    report_as_int = sorted([int(x) for x in filter(lambda x: len(x), expense_report)])
    return report_as_int


def combinations(n: int, r: int) -> int:
    """returns number of combinations n C r"""
    return int(fac(n) / (r * fac(n - r)))


def product(numbers: list):
    return reduce(lambda x, y: x * y, numbers)


def find_pair_sum(num_list: list, sum_target: int) -> list:
    """finds two numbers that sum to the target from the list"""
    j, k = 0, len(num_list) - 1
    current_sum = num_list[j] + num_list[k]
    while current_sum != sum_target and j < k:
        if current_sum < sum_target:
            j += 1
        else:
            j = max(0, j - 1)
            k -= 1
        current_sum = num_list[j] + num_list[k]

    if j > k:
        raise Exception(
            f"Unable to find matching numbers:\n\
                         {j} -> {num_list[j]}\n\
                         {k} -> {num_list[k]}"
        )

    return [num_list[j], num_list[k]]


def find_trio_sum(num_list: list, sum_target: int) -> tuple:
    j, k, l = 0, 1, len(num_list) - 1
    while num_list[k] > sum_target:
        l -= 1

    current_sum = sum([num_list[i] for i in [j, k, l]])
    while current_sum != sum_target and j < k and k < l:
        if k + 1 == l:
            j += 1
            k = j + 1
        elif current_sum < sum_target:
            k += 1
        else:
            j = max(0, j - 1)
            k = j + 1
            l -= 1
        current_sum = sum([num_list[i] for i in [j, k, l]])

    if j >= k or k >= l:
        raise Exception(
            f"Unable to find matching numbers:\n\
                         {j} -> {num_list[j]}\n\
                         {k} -> {num_list[k]}\n\
                         {l} -> {num_list[l]}"
        )

    return [num_list[j], num_list[k], num_list[l]]


if __name__ == "__main__":
    pair_nums = find_pair_sum(get_report(INPUT_FILE), 2020)
    print(product(pair_nums))
    trio_nums = find_trio_sum(get_report(INPUT_FILE), 2020)
    print(product(trio_nums))
