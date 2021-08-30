# Train report for javascript / file:///tmp/top-repos-quality-repos-qjprgsnd/reactor.js.git HEAD 8b27230d591d77df490bac2bb9762b2201315f9a

### Classification report

PPCR: 0.606

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.991| 0.996| 0.835| 0.993| 0.907| 3561| 4245| 0.839 |
| `␣` | 0.964| 0.975| 0.318| 0.969| 0.478| 596| 1828| 0.326 |
| `⏎` | 1.000| 0.979| 0.292| 0.989| 0.452| 233| 781| 0.298 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 203| 0.054 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 210| 0.029 |
| `weighted avg` | 0.984| 0.988| 0.599| 0.986| 0.698| 4407| 7267| 0.606 |
| `micro avg` | 0.988| 0.988| 0.599| 0.988| 0.746| 4407| 7267| 0.606 |
| `macro avg` | 0.591| 0.590| 0.289| 0.590| 0.367| 4407| 7267| 0.606 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|684 |3545 |16 |0 |0 |0 |
|1232 |15 |581 |0 |0 |0 |
|548 |2 |3 |228 |0 |0 |
|204 |6 |0 |0 |0 |0 |
|192 |8 |3 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| reactor.js | 36 |
| test.js | 17 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.5903419127558976, "precision": 0.5909693701514798, "recall": 0.5897759734774304, "support": 4407}, "micro avg": {"f1-score": 0.9879736782391649, "precision": 0.9879736782391649, "recall": 0.9879736782391649, "support": 4407}, "weighted avg": {"f1-score": 0.9860744910834978, "precision": 0.9842036358690998, "recall": 0.9879736782391649, "support": 4407}, "\u2205": {"f1-score": 0.993414599971977, "precision": 0.991331096196868, "recall": 0.9955068800898624, "support": 3561}, "\u23ce": {"f1-score": 0.9891540130151844, "precision": 1.0, "recall": 0.9785407725321889, "support": 233}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.9691409507923269, "precision": 0.9635157545605307, "recall": 0.9748322147651006, "support": 596}},
  "cl_report_full": {"macro avg": {"f1-score": 0.3672917787049204, "precision": 0.5909693701514798, "recall": 0.2889734469020494, "support": 7267}, "micro avg": {"f1-score": 0.7459311290046257, "precision": 0.9879736782391649, "recall": 0.5991468281271501, "support": 7267}, "weighted avg": {"f1-score": 0.6983576923479314, "precision": 0.9289262835685089, "recall": 0.5991468281271501, "support": 7267}, "\u2205": {"f1-score": 0.9065336913438179, "precision": 0.991331096196868, "recall": 0.8351001177856302, "support": 4245}, "\u23ce": {"f1-score": 0.4519326065411298, "precision": 1.0, "recall": 0.2919334186939821, "support": 781}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 210}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 203}, "\u2423": {"f1-score": 0.4779925956396544, "precision": 0.9635157545605307, "recall": 0.31783369803063455, "support": 1828}},
  "ppcr": 0.6064400715563506
}
```
</details>
