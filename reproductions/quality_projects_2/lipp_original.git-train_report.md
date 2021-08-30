# Train report for javascript / file:///tmp/top-repos-quality-repos-det8075v/lipp_original.git HEAD 0a004465cb7c5a7ab028ee2f71aa00f231a20f13

### Classification report

PPCR: 0.973

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.997| 0.995| 0.981| 0.996| 0.989| 91668| 92934| 0.986 |
| `␣` | 0.980| 0.991| 0.946| 0.985| 0.963| 41659| 43644| 0.955 |
| `'` | 0.994| 1.000| 0.992| 0.997| 0.993| 25565| 25775| 0.992 |
| `⏎` | 0.964| 0.983| 0.938| 0.973| 0.951| 15549| 16288| 0.955 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.871| 0.949| 0.889| 0.908| 0.880| 4416| 4716| 0.936 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.860| 0.972| 0.928| 0.913| 0.893| 4346| 4551| 0.955 |
| `⏎␣⁻␣⁻` | 0.996| 0.601| 0.583| 0.750| 0.736| 1355| 1398| 0.969 |
| `⏎␣⁺␣⁺` | 0.943| 0.604| 0.563| 0.736| 0.705| 1333| 1429| 0.933 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 403| 633| 0.637 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 166| 166| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 75| 81| 0.926 |
| `micro avg` | 0.983| 0.983| 0.957| 0.983| 0.970| 186535| 191615| 0.973 |
| `macro avg` | 0.691| 0.645| 0.620| 0.660| 0.646| 186535| 191615| 0.973 |
| `weighted avg` | 0.980| 0.983| 0.957| 0.981| 0.967| 186535| 191615| 0.973 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1266 |91178 |490 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1985 |110 |41279 |0 |103 |85 |73 |8 |1 |0 |0 |0 |
|210 |0 |0 |25565 |0 |0 |0 |0 |0 |0 |0 |0 |
|739 |55 |195 |0 |15280 |19 |0 |0 |0 |0 |0 |0 |
|300 |37 |125 |0 |21 |4192 |0 |41 |0 |0 |0 |0 |
|205 |46 |30 |0 |44 |0 |4224 |0 |2 |0 |0 |0 |
|96 |5 |5 |0 |0 |518 |0 |805 |0 |0 |0 |0 |
|43 |3 |0 |0 |0 |0 |537 |0 |815 |0 |0 |0 |
|230 |0 |2 |0 |401 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |166 |0 |0 |0 |0 |0 |0 |0 |0 |
|6 |0 |0 |0 |0 |0 |75 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| app/static/libraries/glide@3.3.0/glide.js | 1449 |
| app/static/libraries/summernote@0.8.11/summernote-lite.js | 498 |
| app/static/libraries/summernote@0.8.11/summernote-bs4.js | 435 |
| app/static/libraries/summernote@0.8.11/summernote.js | 427 |
| app/static/libraries/summernote@0.8.11/plugin/specialchars/summernote-ext-specialchars.js | 143 |
| app/static/libraries/summernote@0.8.11/plugin/databasic/summernote-ext-databasic.js | 118 |
| app/static/libraries/timeline/timeline.js | 22 |
| app/static/libraries/summernote@0.8.11/plugin/hello/summernote-ext-hello.js | 19 |
| app/static/libraries/summernote@0.8.11/lang/summernote-ko-KR.js | 2 |
| app/static/libraries/summernote@0.8.11/lang/summernote-es-EU.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 166}, "\u0027": {"f1-score": 0.996763880224579, "precision": 0.993548637829855, "recall": 1.0, "support": 25565}, "macro avg": {"f1-score": 0.6598895446537498, "precision": 0.6913591367915626, "recall": 0.6449830326425556, "support": 186535}, "micro avg": {"f1-score": 0.9828611252579945, "precision": 0.9828611252579945, "recall": 0.9828611252579945, "support": 186535}, "weighted avg": {"f1-score": 0.9807045253607465, "precision": 0.9800579396194667, "recall": 0.9828611252579945, "support": 186535}, "\u2205": {"f1-score": 0.9959257681510851, "precision": 0.9972001662401295, "recall": 0.9946546232054806, "support": 91668}, "\u23ce": {"f1-score": 0.9733104019364289, "precision": 0.9640986813048141, "recall": 0.9826998520805197, "support": 15549}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 403}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7361682670324645, "precision": 0.9426229508196722, "recall": 0.6039009752438109, "support": 1333}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9083423618634886, "precision": 0.870793518903199, "recall": 0.9492753623188406, "support": 4416}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7501150483202944, "precision": 0.9963325183374083, "recall": 0.6014760147601476, "support": 1355}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.912803889789303, "precision": 0.8604603788959054, "recall": 0.9719282098481362, "support": 4346}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 75}, "\u2423": {"f1-score": 0.9853553738736052, "precision": 0.9798936523762047, "recall": 0.9908783216111765, "support": 41659}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 166}, "\u0027": {"f1-score": 0.9926998796256747, "precision": 0.993548637829855, "recall": 0.9918525703200776, "support": 25775}, "macro avg": {"f1-score": 0.6462554721299547, "precision": 0.6913591367915626, "recall": 0.6200205622415907, "support": 191615}, "micro avg": {"f1-score": 0.9696575433029221, "precision": 0.9828611252579945, "recall": 0.9568040080369491, "support": 191615}, "weighted avg": {"f1-score": 0.9668035477533911, "precision": 0.9786015330307355, "recall": 0.9568040080369491, "support": 191615}, "\u2205": {"f1-score": 0.989087043304695, "precision": 0.9972001662401295, "recall": 0.9811048701228829, "support": 92934}, "\u23ce": {"f1-score": 0.9509288359212124, "precision": 0.9640986813048141, "recall": 0.9381139489194499, "support": 16288}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 633}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7052124397722295, "precision": 0.9426229508196722, "recall": 0.56333100069979, "support": 1429}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.8797481636935992, "precision": 0.870793518903199, "recall": 0.8888888888888888, "support": 4716}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7355595667870036, "precision": 0.9963325183374083, "recall": 0.5829756795422032, "support": 1398}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8930232558139534, "precision": 0.8604603788959054, "recall": 0.9281476598549769, "support": 4551}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 81}, "\u2423": {"f1-score": 0.9625510085111344, "precision": 0.9798936523762047, "recall": 0.9458115663092292, "support": 43644}},
  "ppcr": 0.9734885055971609
}
```
</details>
