#!/bin/bash

TEMP_FILE="temp.txt"
FINAL_TEMP="final_temp.txt"
FINAL_OUTPUT="n4_00.txt"
TARGET_COUNT=5
MAX_COST=500
CURRENT_COUNT=0
RUN_COUNT=0

# 將舊的 temp.txt 移動到 final_temp.txt
if [ -f "$TEMP_FILE" ]; then
    mv "$TEMP_FILE" "$FINAL_TEMP"
fi

# 清空文件
echo "" > "$TEMP_FILE"
echo "" > "$FINAL_OUTPUT"

# 函數：將輸出同時顯示在螢幕和寫入 temp.txt
log() {
    echo "$1" | tee -a "$TEMP_FILE"
}

log "開始執行實驗，目標：找到 $TARGET_COUNT 個成本 < $MAX_COST 的解"

while [ $CURRENT_COUNT -lt $TARGET_COUNT ]; do
    RUN_COUNT=$((RUN_COUNT + 1))
    
    # 執行程序並捕獲輸出
    log "--- 執行第 $RUN_COUNT 次 ---"
    
    # 執行 cargo build 並隱藏輸出
    cargo build >/dev/null 2>&1
    
    # 執行程序並捕獲輸出
    OUTPUT=$(target/debug/rustabu 0 2>&1)
    
    # 將完整輸出寫入 final_temp.txt
    echo "$OUTPUT" >> "$FINAL_TEMP"
    
    # 提取最終的 makespan (macOS 相容寫法)
    MAKESPAN=$(echo "$OUTPUT" | grep '最優解 makespan:' | grep -oE '[0-9]+\.[0-9]+')
    
    # 檢查 makespan 是否小於目標值
    if (( $(echo "$MAKESPAN < $MAX_COST" | bc -l) )); then
        CURRENT_COUNT=$((CURRENT_COUNT + 1))
        log "找到符合條件的解 #$CURRENT_COUNT: makespan = $MAKESPAN"
        
        # 將符合條件的解寫入最終輸出文件
        if [ $CURRENT_COUNT -eq 1 ]; then
            echo "--- 0" > "$FINAL_OUTPUT"
        else
            echo -e "\n--- $((CURRENT_COUNT-1))" >> "$FINAL_OUTPUT"
        fi
        
        # 過濾並寫入最終輸出
        echo "$OUTPUT" | grep -v '^\s*$' | grep -v '^    Finished\\|^     Running' >> "$FINAL_OUTPUT"
        
        # 保存 convergence.png
        if [ -f "convergence.png" ]; then
            cp "convergence.png" "convergence_${CURRENT_COUNT}.png"
            log "已保存收斂曲線: convergence_${CURRENT_COUNT}.png"
        fi
    else
        # 如果不符合條件，重置計數器
        if [ $CURRENT_COUNT -gt 0 ]; then
            log "中斷連續成功，重置計數器"
            CURRENT_COUNT=0
            echo "" > "$FINAL_OUTPUT"
        fi
        log "跳過，makespan ($MAKESPAN) 大於 $MAX_COST"
    fi
done

log "\n實驗完成！"
log "總共執行了 $RUN_COUNT 次，找到 $TARGET_COUNT 個連續符合條件的解"
log "結果已保存到 $FINAL_OUTPUT"
log "完整執行記錄已保存到 $FINAL_TEMP"
