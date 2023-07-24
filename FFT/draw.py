import numpy as np
import matplotlib.pyplot as plt
import pandas as pd

filename = 'output'

def read_csv_as_bin(filename: str) -> pd.DataFrame:
    df = pd.read_csv(filename, header=None)
    df = df.applymap(lambda x: str(x))
    df_real = df.applymap(lambda x: x[:16])
    df_imag = df.applymap(lambda x: x[16:])
    return df_real, df_imag

def fixed2float(x: int, m: int) -> float:
    if len(x) == 16 and x[0] == '1':
        x = int(x, 2) - (1 << 16)
    else:
        x = int(x, 2)
    return x / (2 ** m)
    
def main():
    df_real, df_imag = read_csv_as_bin(filename + '.txt')
    print(df_real)
    print(df_imag)
    df_real = df_real.applymap(lambda x: fixed2float(x, 11))
    df_imag = df_imag.applymap(lambda x: fixed2float(x, 11))
    df = (df_real ** 2 + df_imag ** 2) ** 0.5
    df.plot()
    plt.xlabel('Frequency')
    plt.ylabel('Amplitude')
    plt.savefig(filename + '.png')

if __name__ == '__main__':
    main()