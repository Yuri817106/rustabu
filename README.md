# rustabu

本專案為演化式計算課程期末報告，計算老師所提供之問題(./src/P4/ 裡面有詳細問題結構)的最低成本排程。

## 如何使用

1. 安裝 Rust<br>
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ``` 
2. 下載和執行本專案：
   ```bash
   git clone https://github.com/Yuri817106/rustabu.git
   cd rustabu
   
   cargo run
   ```

## 研究目的與動機
本專案使用 Tabu Search 的方式來求解老師所給的問題，並使用 Plotters 來畫出收斂曲線。
對於 Tabu List 的設計則是我的動機，我曾經在課堂上提出過使用bit mask的方法來表示Tabu List，所以這專案就是驗證我所提之方法的可行性。

## 問題編碼
對於問題和解我用了結構體表示
```Rust
pub struct Problem {
    pub processor_count: usize,
    pub task_count: usize,
    pub edge_count: usize,
    pub comm_rate: Vec<Vec<f64>>,
    pub comp_cost: Vec<Vec<f64>>,
    pub trans_data_vol: Vec<(usize, usize, f64)>,
}
```
> 問題的結構

```Rust
pub struct Solution {
    pub ss: Vec<usize>,
    pub ms: Vec<usize>,
}
```
> 解的結構

## Tabu List 的設計
我使用 bit mask 的方式來表示 Tabu List，Tabu List 的長度為 問題之工作個數 / 3。
舉個例子：
```
[0, 1, 2, 3, 4] -> [0, 1, 2, 4, 3]
```
我交換了 數值3 和 數值4 的位置，所以 bit mask 就是 00011，並將這個 bit mask 推入 Tabu List。
之後的擾動如果他的 bit mask 是 00011 或是其他已經在 Tabu List 中的 bit mask，都會拒絕接受，除非解品質比目前的最佳解還好。

## 評估函式

## 程式之活動圖

## 非法解的問題
我在擾動的過程中，是一定有機會產生非法解的，所以我對於非法解將會給予其一個很大的懲罰分數，並檢查是否大於等於這個懲罰分數，如果是，則拒絕接受這個解，並重新產生一個新的解。

## 實驗執行之環境
OS: macOS Sequoia 15.5 arm64<br>
CPU: Apple M4 (10) @ 4.46 GHz<br>
GPU: Apple M4 (10) @ 1.58 GHz<br>
RAM: 16.00 GiB<br>
Rust version: 1.87.0<br>

## 實驗參數


## 實驗結果
| 問題編號 | 最佳解 | 最差解 | 平均解 | 標準差 | 執行時間 |
| :--- | :--- | :--- | :--- | :--- | :--- |
| n4_00 |  |  |  |  |  |
| n4_02 |  |  |  |  |  |
| n4_04 |  |  |  |  |  |
| n4_06 |  |  |  |  |  |
