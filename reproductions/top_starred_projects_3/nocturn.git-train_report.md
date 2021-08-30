# Train report for javascript / file:///tmp/top-repos-quality-repos-7nnx1_qs/nocturn.git HEAD 0add6d272361cb09322fd40c475c29784a144267

### Classification report

PPCR: 0.514

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 1.000| 0.744| 0.993| 0.848| 6954| 9350| 0.744 |
| `'` | 1.000| 1.000| 0.912| 1.000| 0.954| 1109| 1216| 0.912 |
| `␣` | 1.000| 0.922| 0.142| 0.959| 0.249| 719| 4657| 0.154 |
| `⏎␣⁻␣⁻` | 1.000| 0.964| 0.415| 0.982| 0.586| 280| 651| 0.430 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 699| 0.036 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 782| 0.009 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 336| 0.000 |
| `weighted avg` | 0.986| 0.989| 0.509| 0.987| 0.601| 9094| 17691| 0.514 |
| `micro avg` | 0.989| 0.989| 0.509| 0.989| 0.672| 9094| 17691| 0.514 |
| `macro avg` | 0.569| 0.555| 0.316| 0.562| 0.377| 9094| 17691| 0.514 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2396 |6954 |0 |0 |0 |0 |0 |0 |
|3938 |56 |663 |0 |0 |0 |0 |0 |
|107 |0 |0 |1109 |0 |0 |0 |0 |
|775 |7 |0 |0 |0 |0 |0 |0 |
|674 |25 |0 |0 |0 |0 |0 |0 |
|371 |10 |0 |0 |0 |0 |270 |0 |
|336 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| gulpfile.js | 18 |
| src/media-popup.js | 16 |
| src/utils/rich-state.js | 8 |
| src/utils/twitter-client.js | 8 |
| src/utils/global-key-bind.js | 6 |
| src/components/tweet.js | 4 |
| src/components/editor.js | 4 |
| src/utils/time-refresher.js | 3 |
| src/components/retweet.js | 3 |
| src/containers/favorite-container.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1109}, "macro avg": {"f1-score": 0.5620428852074252, "precision": 0.5694433190179078, "recall": 0.5551999659390877, "support": 9094}, "micro avg": {"f1-score": 0.9892236639542555, "precision": 0.9892236639542555, "recall": 0.9892236639542555, "support": 9094}, "weighted avg": {"f1-score": 0.9873672043564761, "precision": 0.9858546165772724, "recall": 0.9892236639542555, "support": 9094}, "\u2205": {"f1-score": 0.9930029987148365, "precision": 0.9861032331253545, "recall": 1.0, "support": 6954}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9818181818181818, "precision": 1.0, "recall": 0.9642857142857143, "support": 280}, "\u2423": {"f1-score": 0.9594790159189579, "precision": 1.0, "recall": 0.9221140472878998, "support": 719}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9539784946236559, "precision": 1.0, "recall": 0.9120065789473685, "support": 1216}, "macro avg": {"f1-score": 0.3767844579544281, "precision": 0.5694433190179078, "recall": 0.31612325264138874, "support": 17691}, "micro avg": {"f1-score": 0.6717192458465558, "precision": 0.9892236639542555, "recall": 0.5085071505285174, "support": 17691}, "weighted avg": {"f1-score": 0.600913989587827, "precision": 0.8899477265119023, "recall": 0.5085071505285174, "support": 17691}, "\u2205": {"f1-score": 0.8479453725155469, "precision": 0.9861032331253545, "recall": 0.7437433155080214, "support": 9350}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 782}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 336}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 699}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.5863192182410423, "precision": 1.0, "recall": 0.4147465437788018, "support": 651}, "\u2423": {"f1-score": 0.24924812030075186, "precision": 1.0, "recall": 0.1423663302555293, "support": 4657}},
  "ppcr": 0.5140466904075519
}
```
</details>
