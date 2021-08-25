# Model report for file:///tmp/top-repos-quality-repos-gspqo8he/nwa.git HEAD 4eff6032326d9aa0065ce04abc05148ce8a87fa9

### Dump

```json
{'created_at': '2021-08-21 09:41:22',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.9 kB',
 'tags': [],
 'uuid': 'eaa28d22-ce99-463d-80a1-872005523a92',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-gspqo8he/nwa.git 4eff6032326d9aa0065ce04abc05148ce8a87fa9

# javascript
18 rules, avg.len. 6.4
## train
PPCR: 0.934067
### report
macro
{'f1-score': 0.5402314586977771,
 'precision': 0.596117152155095,
 'recall': 0.5131491468076343,
 'support': 20627}
micro
{'f1-score': 0.9652397343287924,
 'precision': 0.9652397343287924,
 'recall': 0.9652397343287924,
 'support': 20627}
weighted
{'f1-score': 0.9586555464229748,
 'precision': 0.956266004914587,
 'recall': 0.9652397343287924,
 'support': 20627}
### report_full
macro
{'f1-score': 0.49459635480771624,
 'precision': 0.596117152155095,
 'recall': 0.45559348962385554,
 'support': 22083}
micro
{'f1-score': 0.9323343479278857,
 'precision': 0.9652397343287924,
 'recall': 0.9015985146945614,
 'support': 22083}
weighted
{'f1-score': 0.9167539288775973,
 'precision': 0.9447285434003923,
 'recall': 0.9015985146945614,
 'support': 22083}
## test
PPCR: 0.879000
### report
macro
{'f1-score': 0.5058970941417329,
 'precision': 0.5691585702852078,
 'recall': 0.48337753263320105,
 'support': 5027}
micro
{'f1-score': 0.9259996021483986,
 'precision': 0.9259996021483986,
 'recall': 0.9259996021483986,
 'support': 5027}
weighted
{'f1-score': 0.9178970868207074,
 'precision': 0.9184028288462195,
 'recall': 0.9259996021483986,
 'support': 5027}
### report_full
macro
{'f1-score': 0.4573284348901726,
 'precision': 0.5691585702852078,
 'recall': 0.4187519854638804,
 'support': 5719}
micro
{'f1-score': 0.8663688814442584,
 'precision': 0.9259996021483986,
 'recall': 0.813953488372093,
 'support': 5719}
weighted
{'f1-score': 0.8466864515123699,
 'precision': 0.9017651331973848,
 'recall': 0.813953488372093,
 'support': 5719}
```

## javascript
### Summary
14 rules, avg.len. 5.5

| | |
|-|-|
|Min support|93|
|Max support|7848|
|Min confidence|0.9342105388641357|
|Max confidence|0.9968944191932678|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.958. Support: 1467.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 285.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.length ≥ 2<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 188.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.length ≤ 2<br>	∧ +1.roles in {STRING}<br>⇒ y = "<br>Confidence: 0.951. Support: 1473.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.990. Support: 7848.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ⏎<br>Confidence: 0.967. Support: 137.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.996. Support: 128.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.reserved = =<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 114.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 93.` |
| 10 | `  •••start_col ≤ 20<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 170.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 1982.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 447.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.977. Support: 108.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 161.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.5, "max_conf": 0.9968944191932678, "max_support": 7848, "min_conf": 0.9342105388641357, "min_support": 93, "num_rules": 14}}
```
</details>
