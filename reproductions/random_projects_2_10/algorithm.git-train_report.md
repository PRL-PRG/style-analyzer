# Train report for javascript / file:///tmp/top-repos-quality-repos-f4lo97qx/algorithm.git HEAD 4cd2f5ca51357494fafd058c105264bd8f02a7b3

### Classification report

PPCR: 0.480

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.992| 0.761| 0.989| 0.859| 2266| 2952| 0.768 |
| `␣` | 0.950| 0.978| 0.193| 0.964| 0.321| 368| 1866| 0.197 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 142| 0.141 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 350| 0.009 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 112| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 112| 0.000 |
| `macro avg` | 0.323| 0.328| 0.159| 0.325| 0.197| 2657| 5534| 0.480 |
| `weighted avg` | 0.973| 0.981| 0.471| 0.977| 0.567| 2657| 5534| 0.480 |
| `micro avg` | 0.981| 0.981| 0.471| 0.981| 0.637| 2657| 5534| 0.480 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|686 |2247 |19 |0 |0 |0 |0 |
|1498 |8 |360 |0 |0 |0 |0 |
|347 |3 |0 |0 |0 |0 |0 |
|122 |20 |0 |0 |0 |0 |0 |
|112 |0 |0 |0 |0 |0 |0 |
|112 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| leetcode/155-Min Stack/JavaScript/155-min-stack.js | 5 |
| leetcode/160-Intersection of Two Linked Lists/JavaScript/160-intersection-of-two-linked-lists.js | 5 |
| leetcode/64-Minimum Paths Sum/JavaScript/64_minimum_paths_sum.js | 4 |
| leetcode/125-Valid Palindrome/JavaScript/125-valid-palindrome.js | 4 |
| jackdan_javascript/bubble_sort.js | 3 |
| document/sort/test.js | 3 |
| leetcode/49-Group Anagrams/JavaScript/49-group-anagrams.js | 3 |
| leetcode/169-Majority Element/JavaScript/169-majority-element.js | 3 |
| leetcode/122-Best Time to Buy and Sell Stock II/JavaScript/122-best-time-to-buy-and-sell-stock-II.js | 2 |
| leetcode/62-Unique Paths/JavaScript/62-unique-paths.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.3254753167599977, "precision": 0.3227099409054371, "recall": 0.32831267508346446, "support": 2657}, "micro avg": {"f1-score": 0.9811817839668799, "precision": 0.9811817839668799, "recall": 0.9811817839668799, "support": 2657}, "weighted avg": {"f1-score": 0.9769532616889286, "precision": 0.9727944118662729, "recall": 0.9811817839668799, "support": 2657}, "\u2205": {"f1-score": 0.9889964788732394, "precision": 0.9863915715539947, "recall": 0.9916151809355693, "support": 2266}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9638554216867469, "precision": 0.9498680738786279, "recall": 0.9782608695652174, "support": 368}},
  "cl_report_full": {"macro avg": {"f1-score": 0.19666435290660786, "precision": 0.3227099409054371, "recall": 0.15901748446744918, "support": 5534}, "micro avg": {"f1-score": 0.6365523135148334, "precision": 0.9811817839668799, "recall": 0.47108782074448863, "support": 5534}, "weighted avg": {"f1-score": 0.5665025356062282, "precision": 0.8464549593575915, "recall": 0.47108782074448863, "support": 5534}, "\u2205": {"f1-score": 0.8592734225621416, "precision": 0.9863915715539947, "recall": 0.7611788617886179, "support": 2952}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 350}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 142}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 112}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 112}, "\u2423": {"f1-score": 0.32071269487750553, "precision": 0.9498680738786279, "recall": 0.19292604501607716, "support": 1866}},
  "ppcr": 0.4801228767618359
}
```
</details>
