# メモ

## フラッシュのフォーマットについて

- block もしくはその循環連結リストが必要
  （[参照](https://files.waveshare.com/wiki/common/Rp2350-datasheet.pdf#%5B%7B%22num%22%3A352%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C115%2C718.106%2Cnull%5D)）
- 各 block の詳細は[ここ](https://files.waveshare.com/wiki/common/Rp2350-datasheet.pdf#bootrom-concept-block-loop)に記されている
- `0xfe` という謎のブロックは
    [`PICOBIN_BLOCK_ITEM_2BS_IGNORED`](https://github.com/raspberrypi/pico-sdk/blob/a1438dff1d38bd9c65dbd693f0e5db4b9ae91779/src/common/boot_picobin_headers/include/boot/picobin.h#L50)
    らしい
