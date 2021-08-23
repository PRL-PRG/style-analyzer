# Test report for javascript / file:///tmp/top-repos-quality-repos-iecphu_t/alaio-oracle.git HEAD dc4ba8da61304656df1eded68c814b3f8fe3beed

### Classification report

PPCR: 0.279

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.968| 1.000| 0.413| 0.984| 0.579| 448| 1085| 0.413 |
| `␣` | 1.000| 0.883| 0.155| 0.938| 0.268| 77| 439| 0.175 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 50| 0.080 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 48| 0.042 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 18| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 212| 0.000 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 51| 0.000 |
| `macro avg` | 0.281| 0.269| 0.081| 0.274| 0.121| 531| 1903| 0.279 |
| `micro avg` | 0.972| 0.972| 0.271| 0.972| 0.424| 531| 1903| 0.279 |
| `weighted avg` | 0.961| 0.972| 0.271| 0.966| 0.392| 531| 1903| 0.279 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|637 |448 |0 |0 |0 |0 |0 |0 |
|362 |9 |68 |0 |0 |0 |0 |0 |
|212 |0 |0 |0 |0 |0 |0 |0 |
|46 |4 |0 |0 |0 |0 |0 |0 |
|18 |0 |0 |0 |0 |0 |0 |0 |
|51 |0 |0 |0 |0 |0 |0 |0 |
|46 |2 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "macro avg": {"f1-score": 0.27449508741003503, "precision": 0.28108608454180806, "recall": 0.26901669758812613, "support": 531}, "micro avg": {"f1-score": 0.9717514124293786, "precision": 0.9717514124293786, "recall": 0.9717514124293786, "support": 531}, "weighted avg": {"f1-score": 0.9658082491991832, "precision": 0.9613671584239363, "recall": 0.9717514124293786, "support": 531}, "\u2205": {"f1-score": 0.9835345773874864, "precision": 0.9676025917926566, "recall": 1.0, "support": 448}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.9379310344827586, "precision": 1.0, "recall": 0.8831168831168831, "support": 77}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 50}, "macro avg": {"f1-score": 0.12100799220656107, "precision": 0.28108608454180806, "recall": 0.0811143885873844, "support": 1903}, "micro avg": {"f1-score": 0.42399342645850446, "precision": 0.9717514124293786, "recall": 0.27115081450341566, "support": 1903}, "weighted avg": {"f1-score": 0.39189159472075913, "precision": 0.7823693179690133, "recall": 0.27115081450341566, "support": 1903}, "\u2205": {"f1-score": 0.5788113695090439, "precision": 0.9676025917926566, "recall": 0.4129032258064516, "support": 1085}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 212}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 51}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "\u2423": {"f1-score": 0.2682445759368836, "precision": 1.0, "recall": 0.1548974943052392, "support": 439}},
  "ppcr": 0.279033105622701
}
```
</details>
