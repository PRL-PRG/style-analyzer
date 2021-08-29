# Train report for javascript / file:///tmp/top-repos-quality-repos-2gvy7tpn/socialpunishment.git HEAD 59992a0ba121c39af887002bcb00082b6d44e813

### Classification report

PPCR: 0.849

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.986| 0.990| 0.983| 0.988| 0.984| 73827| 74360| 0.993 |
| `␣` | 0.950| 0.985| 0.961| 0.967| 0.956| 36877| 37795| 0.976 |
| `'` | 0.959| 1.000| 0.635| 0.979| 0.764| 6412| 10096| 0.635 |
| `⏎` | 0.974| 0.630| 0.184| 0.765| 0.310| 2138| 7310| 0.292 |
| `⏎⏎` | 0.954| 0.714| 0.154| 0.817| 0.265| 671| 3108| 0.216 |
| `⏎␣⁻␣⁻` | 1.000| 0.453| 0.093| 0.624| 0.170| 565| 2755| 0.205 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.992| 0.616| 0.161| 0.760| 0.277| 380| 1452| 0.262 |
| `"` | 1.000| 0.042| 0.007| 0.081| 0.015| 284| 1607| 0.177 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 154| 1500| 0.103 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 71| 2817| 0.025 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 39| 101| 0.386 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 83| 0.265 |
| `macro avg` | 0.651| 0.452| 0.265| 0.498| 0.312| 121440| 142984| 0.849 |
| `micro avg` | 0.973| 0.973| 0.826| 0.973| 0.894| 121440| 142984| 0.849 |
| `weighted avg` | 0.971| 0.973| 0.826| 0.969| 0.846| 121440| 142984| 0.849 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎⏎| ⏎⇥⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|533 |73087 |740 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|918 |527 |36318 |0 |32 |0 |0 |0 |0 |0 |0 |0 |0 |
|3684 |0 |0 |6412 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|5172 |166 |603 |0 |1346 |23 |0 |0 |0 |0 |0 |0 |0 |
|2437 |91 |95 |0 |4 |479 |0 |0 |0 |0 |2 |0 |0 |
|2746 |4 |67 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2190 |173 |136 |0 |0 |0 |0 |256 |0 |0 |0 |0 |0 |
|1323 |0 |0 |272 |0 |0 |0 |0 |12 |0 |0 |0 |0 |
|1346 |22 |132 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1072 |78 |68 |0 |0 |0 |0 |0 |0 |0 |234 |0 |0 |
|62 |2 |37 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|61 |0 |22 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| _static_root/bootstrap4/js/bootstrap.bundle.js | 605 |
| _static_root/bootstrap4/js/bootstrap.bundle.e52da3817494.js | 605 |
| _static_root/bootstrap4/js/bootstrap.bb38938b1b90.js | 338 |
| _static_root/django_extensions/js/jquery.autocomplete.js | 180 |
| _static_root/django_extensions/js/jquery.autocomplete.26e55daaf7c5.js | 180 |
| _static_root/otree/js/jquery.animate-colors.06edd9739ece.js | 111 |
| _static_root/otree/js/jquery.animate-colors.js | 111 |
| _static_root/django_extensions/js/jquery.ajaxQueue.ac504621bdd8.js | 70 |
| _static_root/django_extensions/js/jquery.ajaxQueue.js | 70 |
| _static_root/admin/js/inlines.eda404ee376d.js | 52 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.08108108108108109, "precision": 1.0, "recall": 0.04225352112676056, "support": 284}, "\u0027": {"f1-score": 0.9792302993280391, "precision": 0.9593058049072412, "recall": 1.0, "support": 6412}, "macro avg": {"f1-score": 0.4983528220266075, "precision": 0.65124289097009, "recall": 0.4524482211874154, "support": 121440}, "micro avg": {"f1-score": 0.9728590250329381, "precision": 0.9728590250329381, "recall": 0.9728590250329381, "support": 121440}, "weighted avg": {"f1-score": 0.9693916604167113, "precision": 0.970946230731715, "recall": 0.9728590250329381, "support": 121440}, "\u2205": {"f1-score": 0.9878156740574549, "precision": 0.985664194200944, "recall": 0.9899765668386904, "support": 73827}, "\u23ce": {"f1-score": 0.7647727272727273, "precision": 0.9739507959479016, "recall": 0.6295603367633302, "support": 2138}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u23ce": {"f1-score": 0.8167092924126171, "precision": 0.954183266932271, "recall": 0.713859910581222, "support": 671}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 71}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 154}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.6236297198538368, "precision": 1.0, "recall": 0.45309734513274336, "support": 565}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.7597402597402598, "precision": 0.9915254237288136, "recall": 0.6157894736842106, "support": 380}, "\u2423": {"f1-score": 0.9672548105732739, "precision": 0.9502852059239102, "recall": 0.9848415001220273, "support": 36877}},
  "cl_report_full": {"\"": {"f1-score": 0.014823965410747375, "precision": 1.0, "recall": 0.0074673304293715, "support": 1607}, "\u0027": {"f1-score": 0.764243146603099, "precision": 0.9593058049072412, "recall": 0.6351030110935024, "support": 10096}, "macro avg": {"f1-score": 0.31177410996561594, "precision": 0.65124289097009, "recall": 0.2648916995940001, "support": 142984}, "micro avg": {"f1-score": 0.8935951350860738, "precision": 0.9728590250329381, "recall": 0.8262742684496167, "support": 142984}, "weighted avg": {"f1-score": 0.8462881591910097, "precision": 0.9426372676123119, "recall": 0.8262742684496167, "support": 142984}, "\u2205": {"f1-score": 0.9842704195003703, "precision": 0.985664194200944, "recall": 0.9828805809575041, "support": 74360}, "\u23ce": {"f1-score": 0.3097100782328578, "precision": 0.9739507959479016, "recall": 0.18413132694938442, "support": 7310}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 83}, "\u23ce\u23ce": {"f1-score": 0.26537396121883655, "precision": 0.954183266932271, "recall": 0.1541184041184041, "support": 3108}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 101}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2817}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1500}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.17004317502490868, "precision": 1.0, "recall": 0.09292196007259527, "support": 2755}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.2772511848341232, "precision": 0.9915254237288136, "recall": 0.16115702479338842, "support": 1452}, "\u2423": {"f1-score": 0.9555733887624485, "precision": 0.9502852059239102, "recall": 0.960920756713851, "support": 37795}},
  "ppcr": 0.8493257986907626
}
```
</details>
