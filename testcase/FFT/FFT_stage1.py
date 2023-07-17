import pandas as pd
import numpy as np
from matplotlib import pyplot as plt

filename = 'input'
N = 64

def read_csv_as_bin(filename: str) -> pd.DataFrame:
    df = pd.read_csv(filename, header=None)
    df = df.applymap(lambda x: str(x))
    return df

def fixed2float(x: str, m: int) -> float:
    if len(x) == 16 and x[0] == '1':
        x2 = int(x, 2) - (1 << 16)
    else:
        x2 = int(x, 2)
    return x2 / (2 ** m)

def float2fixed(x, m):
    return int(np.round(x * (2 ** m)))

def butterfly(x: complex, y: complex, w: complex) -> tuple:
    a = x + w * y
    b = x - w * y
    return (a, b)

def fft_stage1(x: np.ndarray) -> np.ndarray:
    y = []
    for i in range(N // 2):
        y1, y2 = butterfly(x[i], x[i + N // 2], 1 + 0j)
        y.append(y1)
        y.append(y2)
    return np.array(y)

def fft_stage2(x: np.ndarray) -> np.ndarray:
    y = []
    roter = 0;
    for i in range(N // 2):
        y1, y2 = butterfly(x[i], x[i + N // 2], np.exp(1j * np.pi * (roter % 2) / 2))
        y.append(y1)
        y.append(y2)
        roter += N // 4
    return np.array(y)

def fft_stage3(x: np.ndarray) -> np.ndarray:
    y = []
    roter = 0;
    for i in range(N // 2):
        y1, y2 = butterfly(x[i], x[i + N // 2], np.exp(1j * np.pi * (roter % 4) / 4))
        y.append(y1)
        y.append(y2)
        roter += N // 4
    return np.array(y)

def fft_stage4(x: np.ndarray) -> np.ndarray:
    y = []
    roter = 0;
    for i in range(N // 2):
        y1, y2 = butterfly(x[i], x[i + N // 2], np.exp(1j * np.pi * (roter % 8) / 8))
        y.append(y1)
        y.append(y2)
        roter += N // 4
    return np.array(y)

def fft_stage5(x: np.ndarray) -> np.ndarray:
    y = []
    roter = 0;
    for i in range(N // 2):
        y1, y2 = butterfly(x[i], x[i + N // 2], np.exp(1j * np.pi * (roter % 16) / 16))
        y.append(y1)
        y.append(y2)
        roter += N // 4
    return np.array(y)

def fft_stage6(x: np.ndarray) -> np.ndarray:
    y = []
    roter = 0;
    for i in range(N // 2):
        y1, y2 = butterfly(x[i], x[i + N // 2], np.exp(1j * np.pi * (roter % 32) / 32))
        y.append(y1)
        y.append(y2)
        roter += N // 4
    return np.array(y)

def signed_bin_format(n):
    if n < 0:
        return format((1 << 16) + n, '016b')
    else:
        return format(n, '016b')


def print_bin(x: int, y: int) -> None:
    print(format(x, '016b') + format(y, '016b'))

def print_bin_complex(x: complex) -> None:
    print_bin(float2fixed(x.real, 11), float2fixed(x.imag, 11))

def print_bin_real(x: complex) -> None:
    print(signed_bin_format(float2fixed(x.real, 11)))

def main():
    df = read_csv_as_bin(filename + '.txt')
    df = df.applymap(lambda x: fixed2float(x, 11))
    df = df.applymap(lambda x: x + 0j)
    df = df[0].values
    df = fft_stage1(df)
    df = fft_stage2(df)
    df = fft_stage3(df)
    df = fft_stage4(df)
    df = fft_stage5(df)
    df = fft_stage6(df)
    df = df.tolist()
    for i in range(N):
        print_bin_real(df[i])
    df_real = [x.real for x in df]
    df_imag = [x.imag for x in df]
    plt.plot(df_imag)
    plt.savefig(filename + '_pyfft.png')

if __name__ == '__main__':
    main()