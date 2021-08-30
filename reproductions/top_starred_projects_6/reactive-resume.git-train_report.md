# Train report for javascript / file:///tmp/top-repos-quality-repos-4_ulr_rn/reactive-resume.git HEAD 99d7d3aad24e8a18b304d1563418581e2e9422dc

### Classification report

PPCR: 0.819

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.981| 0.991| 0.915| 0.986| 0.947| 19255| 20870| 0.923 |
| `␣` | 0.965| 0.974| 0.765| 0.970| 0.853| 7593| 9674| 0.785 |
| `'` | 1.000| 1.000| 0.740| 1.000| 0.850| 2439| 3297| 0.740 |
| `"` | 1.000| 1.000| 0.998| 1.000| 0.999| 1326| 1329| 0.998 |
| `⏎` | 0.978| 0.967| 0.461| 0.973| 0.626| 1157| 2429| 0.476 |
| `⏎␣⁺␣⁺` | 0.970| 0.606| 0.287| 0.746| 0.443| 434| 916| 0.474 |
| `⏎␣⁻␣⁻` | 0.981| 0.735| 0.270| 0.840| 0.423| 343| 934| 0.367 |
| `⏎⏎` | 0.990| 0.977| 0.507| 0.984| 0.670| 310| 598| 0.518 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 73| 0.082 |
| `micro avg` | 0.979| 0.979| 0.802| 0.979| 0.882| 32863| 40120| 0.819 |
| `weighted avg` | 0.979| 0.979| 0.802| 0.979| 0.869| 32863| 40120| 0.819 |
| `macro avg` | 0.874| 0.806| 0.549| 0.833| 0.646| 32863| 40120| 0.819 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1615 |19089 |158 |0 |0 |8 |0 |0 |0 |0 |
|2081 |171 |7398 |0 |24 |0 |0 |0 |0 |0 |
|858 |0 |0 |2439 |0 |0 |0 |0 |0 |0 |
|1272 |30 |5 |0 |1119 |0 |0 |0 |3 |0 |
|482 |72 |99 |0 |0 |263 |0 |0 |0 |0 |
|591 |84 |6 |0 |1 |0 |252 |0 |0 |0 |
|3 |0 |0 |0 |0 |0 |0 |1326 |0 |0 |
|288 |7 |0 |0 |0 |0 |0 |0 |303 |0 |
|67 |1 |0 |0 |0 |0 |5 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/contexts/ResumeContext.js | 59 |
| src/contexts/DatabaseContext.js | 34 |
| src/contexts/UserContext.js | 32 |
| functions/index.js | 27 |
| src/hooks/useAuthState.js | 24 |
| src/components/builder/right/sections/Layout.js | 22 |
| src/modals/sections/ExportModal.js | 22 |
| src/pages/app/dashboard.js | 19 |
| src/components/builder/left/LeftSidebar.js | 18 |
| src/contexts/SettingsContext.js | 17 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1326}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2439}, "macro avg": {"f1-score": 0.8331584039648138, "precision": 0.8739606237188615, "recall": 0.8056619743005066, "support": 32863}, "micro avg": {"f1-score": 0.9794906125429814, "precision": 0.9794906125429814, "recall": 0.9794906125429814, "support": 32863}, "weighted avg": {"f1-score": 0.9786291295384539, "precision": 0.979292135866741, "recall": 0.9794906125429814, "support": 32863}, "\u2205": {"f1-score": 0.9862822599395489, "precision": 0.9812377917137863, "recall": 0.9913788626330823, "support": 19255}, "\u23ce": {"f1-score": 0.9726205997392439, "precision": 0.9781468531468531, "recall": 0.9671564390665515, "support": 1157}, "\u23ce\u23ce": {"f1-score": 0.9837662337662337, "precision": 0.9901960784313726, "recall": 0.9774193548387097, "support": 310}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7460992907801418, "precision": 0.9704797047970479, "recall": 0.6059907834101382, "support": 434}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.84, "precision": 0.980544747081712, "recall": 0.7346938775510204, "support": 343}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.969657251458156, "precision": 0.9650404382989826, "recall": 0.9743184512050573, "support": 7593}},
  "cl_report_full": {"\"": {"f1-score": 0.9988700564971752, "precision": 1.0, "recall": 0.9977426636568849, "support": 1329}, "\u0027": {"f1-score": 0.8504184100418409, "precision": 1.0, "recall": 0.7397634212920837, "support": 3297}, "macro avg": {"f1-score": 0.6458203193006524, "precision": 0.8739606237188615, "recall": 0.5490217822915402, "support": 40120}, "micro avg": {"f1-score": 0.8820958305358783, "precision": 0.9794906125429814, "recall": 0.8023180458624127, "support": 40120}, "weighted avg": {"f1-score": 0.8691130356144042, "precision": 0.9773947676404576, "recall": 0.8023180458624127, "support": 40120}, "\u2205": {"f1-score": 0.9467810733062195, "precision": 0.9812377917137863, "recall": 0.9146621945376138, "support": 20870}, "\u23ce": {"f1-score": 0.6263643996641478, "precision": 0.9781468531468531, "recall": 0.46068340881020997, "support": 2429}, "\u23ce\u23ce": {"f1-score": 0.6703539823008849, "precision": 0.9901960784313726, "recall": 0.5066889632107023, "support": 598}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.4431339511373209, "precision": 0.9704797047970479, "recall": 0.287117903930131, "support": 916}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.42317380352644834, "precision": 0.980544747081712, "recall": 0.2698072805139186, "support": 934}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 73}, "\u2423": {"f1-score": 0.853287197231834, "precision": 0.9650404382989826, "recall": 0.7647302046723176, "support": 9674}},
  "ppcr": 0.8191176470588235
}
```
</details>
