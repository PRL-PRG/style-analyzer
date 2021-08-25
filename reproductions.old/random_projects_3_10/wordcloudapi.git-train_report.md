# Train report for javascript / file:///tmp/top-repos-quality-repos-wxj2jv0i/wordcloudapi.git HEAD bc229db8089f57c3ac2f310674232575f6a5b7f8

### Classification report

PPCR: 0.569

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.943| 1.000| 0.845| 0.970| 0.891| 3387| 4006| 0.845 |
| `"` | 1.000| 1.000| 0.500| 1.000| 0.667| 349| 698| 0.500 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 68| 1400| 0.049 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 63| 225| 0.280 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 38| 360| 0.106 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 37| 244| 0.152 |
| `weighted avg` | 0.898| 0.948| 0.539| 0.922| 0.582| 3942| 6933| 0.569 |
| `micro avg` | 0.948| 0.948| 0.539| 0.948| 0.687| 3942| 6933| 0.569 |
| `macro avg` | 0.324| 0.333| 0.224| 0.328| 0.260| 3942| 6933| 0.569 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|619 |3387 |0 |0 |0 |0 |0 |
|1332 |68 |0 |0 |0 |0 |0 |
|349 |0 |0 |349 |0 |0 |0 |
|322 |38 |0 |0 |0 |0 |0 |
|207 |37 |0 |0 |0 |0 |0 |
|162 |63 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| landingpage/src/components/contact.js | 38 |
| landingpage/src/components/navlinks.js | 23 |
| landingpage/src/components/seo.js | 15 |
| landingpage/src/components/items-blog.js | 15 |
| landingpage/gatsby-config.js | 12 |
| landingpage/src/components/sectiontitle.js | 12 |
| landingpage/src/components/pagination.js | 11 |
| landingpage/src/components/navbar.js | 9 |
| landingpage/src/components/items-portfolio.js | 9 |
| landingpage/src/templates/blog.js | 7 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 349}, "macro avg": {"f1-score": 0.32841451766953195, "precision": 0.32377771592912147, "recall": 0.3333333333333333, "support": 3942}, "micro avg": {"f1-score": 0.9477422628107559, "precision": 0.9477422628107559, "recall": 0.9477422628107559, "support": 3942}, "weighted avg": {"f1-score": 0.9223845327448578, "precision": 0.8984806552794535, "recall": 0.9477422628107559, "support": 3942}, "\u2205": {"f1-score": 0.9704871060171919, "precision": 0.9426662955747286, "recall": 1.0, "support": 3387}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 37}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 63}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 68}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 698}, "macro avg": {"f1-score": 0.2596832916611835, "precision": 0.32377771592912147, "recall": 0.2242469628889998, "support": 6933}, "micro avg": {"f1-score": 0.6870804597701149, "precision": 0.9477422628107559, "recall": 0.5388720611567864, "support": 6933}, "weighted avg": {"f1-score": 0.5822031249148815, "precision": 0.6453658127898979, "recall": 0.5388720611567864, "support": 6933}, "\u2205": {"f1-score": 0.8914330833004342, "precision": 0.9426662955747286, "recall": 0.845481777333999, "support": 4006}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 360}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 244}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 225}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1400}},
  "ppcr": 0.5685850281263523
}
```
</details>
