#!/bin/bash

TEMP_FILE="temp.txt"
FINAL_TEMP="final_temp.txt"
FINAL_OUTPUT="n4_00.txt"
TARGET_COUNT=5
MAX_COST=500
TOTAL_RUNS=0
BATCH_SIZE=5

# 清空文件
echo "" > "$TEMP_FILE"
echo "" > "$FINAL_TEMP"
echo "" > "$FINAL_OUTPUT"

# 函數：將輸出同時顯示在螢幕和寫入 temp.txt
log() {
    echo "$1" | tee -a "$TEMP_FILE"
}

log "開始執行實驗，目標：找到 $TARGET_COUNT 個成本 < $MAX_COST 的解"

# 初始化最終輸出文件
echo "--- 0" > "$FINAL_OUTPUT"

while true; do
    # 清空臨時結果數組
    BATCH_RESULTS=()
    BATCH_MAKESPANS=()
    BATCH_STATUS=()  # 記錄每次執行的狀態
    
    log "=== 開始新一輪批次執行（每輪 $BATCH_SIZE 次）==="
    
    # 執行 BATCH_SIZE 次
    for ((i=1; i<=$BATCH_SIZE; i++)); do
        TOTAL_RUNS=$((TOTAL_RUNS + 1))
        RUN_NUMBER=$TOTAL_RUNS
        
        # 執行 cargo build 並隱藏輸出
        cargo build >/dev/null 2>&1
        
        # 執行程序並捕獲輸出
        OUTPUT=$(target/debug/rustabu 0 2>&1)
        
        # 將完整輸出寫入 final_temp.txt
        echo "$OUTPUT" >> "$FINAL_TEMP"
        echo "" >> "$FINAL_TEMP"
        
        # 提取最終的 makespan (macOS 相容寫法)
        MAKESPAN=$(echo "$OUTPUT" | grep '最優解 makespan:' | grep -oE '[0-9]+\.[0-9]+')
        
        # 檢查 makespan 是否小於目標值
        if [ -z "$MAKESPAN" ] || (( $(echo "$MAKESPAN >= $MAX_COST" | bc -l) )); then
            BATCH_STATUS+=("失敗")
        else
            BATCH_STATUS+=("成功")
        fi
        
        # 保存結果
        BATCH_RESULTS+=("$OUTPUT")
        BATCH_MAKESPANS+=("${MAKESPAN:-無效值}")
    done
    
    # 批次執行完成，輸出結果摘要
    log "本輪批次執行完成，結果如下："
    ALL_VALID=true
    
    # 檢查每次執行的結果
    for ((i=0; i<BATCH_SIZE; i++)); do
        log "第 $((i+1)) 次執行: makespan = ${BATCH_MAKESPANS[$i]}, 狀態: ${BATCH_STATUS[$i]}"
        
        # 如果有任何一次失敗，整批視為無效
        if [ "${BATCH_STATUS[$i]}" != "成功" ]; then
            ALL_VALID=false
        fi
    done
    
    # 如果本輪批次全部符合條件
    if [ "$ALL_VALID" = true ]; then
        # 將結果寫入最終輸出文件
        for ((i=0; i<${#BATCH_RESULTS[@]}; i++)); do
            echo -e "\n--- $((i + 1))" >> "$FINAL_OUTPUT"
            echo "${BATCH_RESULTS[$i]}" >> "$FINAL_OUTPUT"
        done
        
        log "成功完成一輪批次執行，所有結果符合條件"
        break
    else
        log "本輪批次執行中有不符合條件的結果，將重新開始新一輪批次"
    fi
done

log "實驗完成！總共執行了 $TOTAL_RUNS 次，成功收集到 $BATCH_SIZE 個符合條件的解"
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
