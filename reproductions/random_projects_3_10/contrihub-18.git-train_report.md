# Train report for javascript / file:///tmp/top-repos-quality-repos-wyfvz12f/contrihub-18.git HEAD 48ca8193be096ee4f139073492ae08adab17df3d

### Classification report

PPCR: 0.860

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.993| 0.974| 0.951| 0.984| 0.972| 27497| 28150| 0.977 |
| `␣` | 0.911| 0.992| 0.961| 0.949| 0.935| 14167| 14622| 0.969 |
| `'` | 0.985| 1.000| 0.779| 0.993| 0.870| 5239| 6725| 0.779 |
| `⏎` | 0.931| 0.597| 0.164| 0.728| 0.278| 839| 3063| 0.274 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.918| 0.858| 0.615| 0.887| 0.737| 663| 925| 0.717 |
| `⏎␣⁺␣⁺` | 0.921| 0.973| 0.885| 0.946| 0.903| 659| 724| 0.910 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 230| 956| 0.241 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 114| 708| 0.161 |
| `"` | 1.000| 0.049| 0.005| 0.093| 0.009| 82| 843| 0.097 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 75| 900| 0.083 |
| `weighted avg` | 0.957| 0.964| 0.829| 0.959| 0.852| 49565| 57616| 0.860 |
| `micro avg` | 0.964| 0.964| 0.829| 0.964| 0.892| 49565| 57616| 0.860 |
| `macro avg` | 0.666| 0.544| 0.436| 0.558| 0.470| 49565| 57616| 0.860 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|653 |26782 |707 |0 |8 |0 |0 |0 |0 |0 |0 |
|455 |77 |14047 |0 |29 |0 |0 |0 |11 |3 |0 |
|1486 |0 |0 |5239 |0 |0 |0 |0 |0 |0 |0 |
|2224 |13 |323 |0 |501 |0 |0 |0 |0 |2 |0 |
|761 |0 |0 |78 |0 |4 |0 |0 |0 |0 |0 |
|825 |0 |75 |0 |0 |0 |0 |0 |0 |0 |0 |
|726 |14 |166 |0 |0 |0 |0 |0 |0 |50 |0 |
|262 |38 |56 |0 |0 |0 |0 |0 |569 |0 |0 |
|65 |0 |18 |0 |0 |0 |0 |0 |0 |641 |0 |
|594 |41 |33 |0 |0 |0 |0 |0 |40 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| ContriHub/ContriHub/static_root/Projects/js/bootstrap.4bc939cd6b79.js | 442 |
| ContriHub/ContriHub/static_root/admin/js/urlify.4087f3e18796.js | 114 |
| ContriHub/ContriHub/static_root/admin/js/urlify.js | 114 |
| ContriHub/ContriHub/static_root/admin/js/core.js | 96 |
| ContriHub/ContriHub/static_root/admin/js/core.953326603873.js | 96 |
| ContriHub/ContriHub/static_root/admin/js/inlines.ff88b5c8f54a.js | 88 |
| ContriHub/ContriHub/static_root/admin/js/inlines.js | 88 |
| ContriHub/ContriHub/static_root/admin/js/admin/DateTimeShortcuts.e3407a6ea666.js | 87 |
| ContriHub/ContriHub/static_root/admin/js/admin/DateTimeShortcuts.js | 87 |
| ContriHub/ContriHub/static_root/admin/js/SelectFilter2.bf63c55b78f6.js | 62 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.09302325581395349, "precision": 1.0, "recall": 0.04878048780487805, "support": 82}, "\u0027": {"f1-score": 0.9926108374384237, "precision": 0.9853300733496333, "recall": 1.0, "support": 5239}, "macro avg": {"f1-score": 0.557930168143084, "precision": 0.6659153716609563, "recall": 0.5442352812798047, "support": 49565}, "micro avg": {"f1-score": 0.9640472107333804, "precision": 0.9640472107333804, "recall": 0.9640472107333804, "support": 49565}, "weighted avg": {"f1-score": 0.9588106974905071, "precision": 0.9573812944304967, "recall": 0.9640472107333804, "support": 49565}, "\u2205": {"f1-score": 0.9835114391685945, "precision": 0.9932134248099388, "recall": 0.9739971633269084, "support": 27497}, "\u23ce": {"f1-score": 0.727668845315904, "precision": 0.9312267657992565, "recall": 0.5971394517282479, "support": 839}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 75}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9461254612546127, "precision": 0.9209770114942529, "recall": 0.9726858877086495, "support": 659}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 230}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 114}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.886983632112237, "precision": 0.917741935483871, "recall": 0.8582202111613876, "support": 663}, "\u2423": {"f1-score": 0.9493782103271153, "precision": 0.9106645056726094, "recall": 0.9915296110679749, "support": 14167}},
  "cl_report_full": {"\"": {"f1-score": 0.009445100354191265, "precision": 1.0, "recall": 0.004744958481613286, "support": 843}, "\u0027": {"f1-score": 0.8701212423185517, "precision": 0.9853300733496333, "recall": 0.7790334572490706, "support": 6725}, "macro avg": {"f1-score": 0.47040695344297206, "precision": 0.6659153716609563, "recall": 0.4359916690428796, "support": 57616}, "micro avg": {"f1-score": 0.8916319123725286, "precision": 0.9640472107333804, "recall": 0.8293356012218828, "support": 57616}, "weighted avg": {"f1-score": 0.8517810200337593, "precision": 0.9218287504149628, "recall": 0.8293356012218828, "support": 57616}, "\u2205": {"f1-score": 0.9718588406060056, "precision": 0.9932134248099388, "recall": 0.9514031971580817, "support": 28150}, "\u23ce": {"f1-score": 0.27825603998889203, "precision": 0.9312267657992565, "recall": 0.16356513222331048, "support": 3063}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 900}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9028169014084506, "precision": 0.9209770114942529, "recall": 0.8853591160220995, "support": 724}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 956}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 708}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7365695792880258, "precision": 0.917741935483871, "recall": 0.6151351351351352, "support": 925}, "\u2423": {"f1-score": 0.9350018304656038, "precision": 0.9106645056726094, "recall": 0.9606756941594857, "support": 14622}},
  "ppcr": 0.8602645098583727
}
```
</details>
