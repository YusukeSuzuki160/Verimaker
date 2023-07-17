def int_to_bit(n: int, bit: int) -> str:
    return format(n, '0' + str(bit) + 'b')

def reverse(n: int, bit: int) -> int:
    return int(int_to_bit(n, bit)[::-1], 2)

def main():
    for i in range(64):
        print('assign bit_rev[%d] = 6\'b%s;' % (i, format(reverse(i, 6), '06b')))

if __name__ == '__main__':
    main()