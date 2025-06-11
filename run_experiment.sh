#!/bin/bash

TEMP_FILE="temp.txt"
FINAL_TEMP="final_temp.txt"
FINAL_OUTPUT="n4_00.txt"
TARGET_COUNT=5
MAX_COST=500
TOTAL_RUNS=0
BATCH_SIZE=5

# 清空文件
echo "" > "$FINAL_OUTPUT"

# 編譯程式
cargo build --release >/dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "編譯失敗，請檢查錯誤訊息"
    exit 1
fi

# 函數：將輸出同時顯示在螢幕和寫入 temp.txt
log() {
    echo "$1"
}

log "開始執行實驗，目標：找到 $TARGET_COUNT 個成本 < $MAX_COST 的解"

# 初始化最終輸出文件
touch "$FINAL_OUTPUT"  # 創建空文件，不寫入任何內容

while true; do
    # 清空臨時結果數組
    BATCH_MAKESPANS=()
    BATCH_STATUS=()  # 記錄每次執行的狀態
    BATCH_RESULTS=()
    BATCH_IMAGES=()
    
    log "=== 開始新一輪批次執行 ==="
    
    # 執行 BATCH_SIZE 次
    for ((RUN_COUNT=0; RUN_COUNT<BATCH_SIZE; RUN_COUNT++)); do
        TOTAL_RUNS=$((TOTAL_RUNS + 1))
        RUN_NUMBER=$TOTAL_RUNS
        
        # 設定臨時圖片路徑和最終圖片路徑
        TEMP_IMAGE="temp_convergence_$((RUN_COUNT+1)).png"
        FINAL_IMAGE="convergence_$((RUN_COUNT+1)).png"
        
        # 如果存在舊的臨時圖片，先刪除
        rm -f "$TEMP_IMAGE" 2>/dev/null
        
        # 執行已編譯的二進制文件並捕獲輸出
        OUTPUT=$(./target/release/rustabu -i $INPUT -o $OUTPUT_FILE -m $MAX_ITER -t $TABU_TENURE -p $PERTURB_STRENGTH -s $SEED 2>&1)
        
        # 檢查並重命名圖片
        if [ -f "convergence.png" ]; then
            # 如果目標文件已存在，先刪除
            if [ -f "$TEMP_IMAGE" ]; then
                rm -f "$TEMP_IMAGE"
            fi
            # 移動並重命名圖片
            mv "convergence.png" "$TEMP_IMAGE"
        fi
        
        # 將完整輸出寫入 final_temp.txt
        # echo "$OUTPUT" >> "$FINAL_TEMP"
        # echo "" >> "$FINAL_TEMP"
        
        # 提取最終的 makespan (macOS 相容寫法)
        MAKESPAN=$(echo "$OUTPUT" | grep '最優解 makespan:' | grep -oE '[0-9]+\.[0-9]+')
        
        # 提取整個輸出，移除 cargo 構建和運行的信息
        SOLUTION=$(echo "$OUTPUT" | grep -v '^    Finished' | grep -v '^     Running' | grep -v '^\s*$' | sed 's/^/    /')
        
        # 檢查執行是否成功
        if [ $? -ne 0 ]; then
            log "執行失敗，錯誤信息：$OUTPUT"
            BATCH_STATUS+=("失敗")
            BATCH_MAKESPANS+=("N/A")
            BATCH_RESULTS+=("")
            # 刪除可能生成的臨時圖片
            rm -f "${TEMP_IMAGE}" 2>/dev/null
            continue
        fi
        
        # 檢查圖片是否生成
        if [ -f "${TEMP_IMAGE}" ]; then
            # 將圖片路徑加入數組
            BATCH_IMAGES+=("${TEMP_IMAGE}")
        else
            # 添加一個空字串到數組以保持索引一致
            BATCH_IMAGES+=("")
        fi
        
        # 檢查 makespan 是否小於目標值
        if [ -z "$MAKESPAN" ] || (( $(echo "$MAKESPAN >= $MAX_COST" | bc -l) )); then
            BATCH_STATUS+=("失敗")
        else
            BATCH_STATUS+=("成功")
        fi
        
        # 將結果加入批次
        BATCH_MAKESPANS+=("${MAKESPAN:-N/A}")
        BATCH_RESULTS+=("$SOLUTION")
    done
    
    # 批次執行完成，檢查結果
    ALL_VALID=true
    
    # 檢查每次執行的結果
    for ((i=0; i<BATCH_SIZE; i++)); do
        
        # 如果有任何一次失敗，整批視為無效
        if [ "${BATCH_STATUS[$i]}" != "成功" ]; then
            ALL_VALID=false
        fi
    done
    
    # 如果本輪批次全部符合條件
    if [ "$ALL_VALID" = true ]; then
        # 清空最終輸出文件
        > "$FINAL_OUTPUT"
        
        # 將結果寫入最終輸出文件
        for ((i=0; i<${#BATCH_RESULTS[@]}; i++)); do
            if [ $i -gt 0 ]; then
                echo "" >> "$FINAL_OUTPUT"
            fi
            echo "--- $((i + 1))" >> "$FINAL_OUTPUT"
            echo "${BATCH_RESULTS[$i]}" >> "$FINAL_OUTPUT"
            
            # 保存符合條件的圖片
            if [ -f "${BATCH_IMAGES[$i]}" ]; then
                FINAL_IMAGE="convergence_$((i+1)).png"
                    # 如果目標文件已存在，先刪除
                if [ -f "$FINAL_IMAGE" ]; then
                    rm -f "$FINAL_IMAGE"
                fi
                # 複製圖片到最終位置
                cp "${BATCH_IMAGES[$i]}" "$FINAL_IMAGE"
            else
                log "警告: 找不到圖片文件: ${BATCH_IMAGES[$i]}"
            fi
        done
        
        log "成功完成一輪批次執行，所有結果符合條件"
        break  # 找到符合條件的批次，結束循環
    else
        # 刪除臨時圖片
        for img in "${BATCH_IMAGES[@]}"; do
            rm -f "$img" 2>/dev/null
        done
        log "本輪批次執行中有不符合條件的結果，將重新開始新一輪批次"
    fi
done

# 重命名臨時圖片，移除 temp_ 前綴
for temp_img in temp_convergence_*.png; do
    if [ -f "$temp_img" ]; then
        new_img=${temp_img#temp_}
        mv "$temp_img" "$new_img" 2>/dev/null && log "已重命名圖片: $temp_img -> $new_img"
    fi
done

log "\n實驗完成！"
log "總共執行了 $TOTAL_RUNS 次，成功收集到 $BATCH_SIZE 個符合條件的解"
log "結果已保存到 $FINAL_OUTPUT"
