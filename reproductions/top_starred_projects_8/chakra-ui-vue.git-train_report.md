# Train report for javascript / file:///tmp/top-repos-quality-repos-my3guemq/chakra-ui-vue.git HEAD 8984d3cb11cb5e494fb8abd01046c6f2b77d6118

### Classification report

PPCR: 0.859

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.993| 0.993| 0.913| 0.993| 0.951| 37726| 41016| 0.920 |
| `␣` | 0.970| 0.986| 0.862| 0.978| 0.913| 24500| 28009| 0.875 |
| `'` | 1.000| 1.000| 0.989| 1.000| 0.994| 10289| 10405| 0.989 |
| `⏎` | 0.975| 0.965| 0.676| 0.970| 0.799| 6631| 9462| 0.701 |
| `⏎␣⁻␣⁻` | 0.941| 0.963| 0.726| 0.952| 0.819| 3182| 4223| 0.753 |
| `⏎␣⁺␣⁺` | 0.979| 0.859| 0.409| 0.915| 0.577| 2056| 4317| 0.476 |
| `⏎⏎` | 0.956| 0.760| 0.341| 0.847| 0.503| 691| 1540| 0.449 |
| `"` | 1.000| 1.000| 0.061| 1.000| 0.114| 4| 66| 0.061 |
| `weighted avg` | 0.983| 0.983| 0.845| 0.983| 0.901| 85079| 99038| 0.859 |
| `macro avg` | 0.977| 0.941| 0.622| 0.957| 0.709| 85079| 99038| 0.859 |
| `micro avg` | 0.983| 0.983| 0.845| 0.983| 0.909| 85079| 99038| 0.859 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3290 |37447 |217 |0 |9 |19 |29 |5 |0 |
|3509 |46 |24157 |0 |114 |18 |163 |2 |0 |
|116 |0 |0 |10289 |0 |0 |0 |0 |0 |
|2831 |13 |201 |0 |6399 |1 |0 |17 |0 |
|2261 |116 |172 |0 |1 |1767 |0 |0 |0 |
|1041 |66 |48 |0 |4 |0 |3064 |0 |0 |
|849 |10 |119 |0 |37 |0 |0 |525 |0 |
|62 |0 |0 |0 |0 |0 |0 |0 |4 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| website/components/MDXComponents.js | 84 |
| packages/chakra-ui-core/src/CMenu/CMenu.js | 48 |
| packages/chakra-ui-core/src/CInput/utils/input.styles.js | 44 |
| packages/chakra-ui-core/src/CNumberInput/CNumberInput.js | 41 |
| packages/chakra-ui-core/src/CPopover/CPopover.js | 39 |
| packages/chakra-ui-core/src/CModal/CModal.js | 39 |
| packages/chakra-ui-core/src/CPopper/CPopper.js | 36 |
| packages/chakra-ui-core/src/CSlider/CSlider.js | 29 |
| website/components/ColorPalette.js | 27 |
| packages/chakra-ui-core/src/CTabs/utils/tabs.styles.js | 27 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 4}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 10289}, "macro avg": {"f1-score": 0.9568193145008071, "precision": 0.9767604562537644, "recall": 0.9407172552917364, "support": 85079}, "micro avg": {"f1-score": 0.9832273534009568, "precision": 0.9832273534009568, "recall": 0.9832273534009568, "support": 85079}, "weighted avg": {"f1-score": 0.9830366518623369, "precision": 0.9832694360832415, "recall": 0.9832273534009568, "support": 85079}, "\u2205": {"f1-score": 0.9929730589732711, "precision": 0.9933418218473129, "recall": 0.9926045697927159, "support": 37726}, "\u23ce": {"f1-score": 0.9699128457749149, "precision": 0.9748628884826326, "recall": 0.9650128185793998, "support": 6631}, "\u23ce\u23ce": {"f1-score": 0.8467741935483871, "precision": 0.9562841530054644, "recall": 0.7597684515195369, "support": 691}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9153069153069154, "precision": 0.9789473684210527, "recall": 0.8594357976653697, "support": 2056}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9518484001242622, "precision": 0.941031941031941, "recall": 0.9629164047768699, "support": 3182}, "\u2423": {"f1-score": 0.9777391022787065, "precision": 0.9696154772417115, "recall": 0.986, "support": 24500}},
  "cl_report_full": {"\"": {"f1-score": 0.1142857142857143, "precision": 1.0, "recall": 0.06060606060606061, "support": 66}, "\u0027": {"f1-score": 0.9943945104861313, "precision": 1.0, "recall": 0.9888515136953387, "support": 10405}, "macro avg": {"f1-score": 0.708861834241606, "precision": 0.9767604562537644, "recall": 0.6221214100939785, "support": 99038}, "micro avg": {"f1-score": 0.9086830656593362, "precision": 0.9832273534009568, "recall": 0.8446454896100487, "support": 99038}, "weighted avg": {"f1-score": 0.9009858027781802, "precision": 0.9821360688811883, "recall": 0.8446454896100487, "support": 99038}, "\u2205": {"f1-score": 0.9514698782935692, "precision": 0.9933418218473129, "recall": 0.9129851765164814, "support": 41016}, "\u23ce": {"f1-score": 0.7985773118682142, "precision": 0.9748628884826326, "recall": 0.676284083703234, "support": 9462}, "\u23ce\u23ce": {"f1-score": 0.5026328386787937, "precision": 0.9562841530054644, "recall": 0.3409090909090909, "support": 1540}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.5772623325710552, "precision": 0.9789473684210527, "recall": 0.40931202223766505, "support": 4317}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.81936087712261, "precision": 0.941031941031941, "recall": 0.7255505564764385, "support": 4223}, "\u2423": {"f1-score": 0.9129112106267596, "precision": 0.9696154772417115, "recall": 0.862472776607519, "support": 28009}},
  "ppcr": 0.8590541004462934
}
```
</details>
