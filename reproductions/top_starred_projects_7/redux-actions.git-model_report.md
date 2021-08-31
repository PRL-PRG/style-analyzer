# Model report for file:///tmp/top-repos-quality-repos-3xmgzbxq/redux-actions.git HEAD 4bd68b11b841718e64999d214544d6a87337644e

### Dump

```json
{'created_at': '2021-08-30 18:21:58',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.7 kB',
 'tags': [],
 'uuid': '753df7b5-71e7-482f-8211-bd9b8607fbc9',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-3xmgzbxq/redux-actions.git 4bd68b11b841718e64999d214544d6a87337644e

# javascript
47 rules, avg.len. 5.3
## train
PPCR: 0.897673
### report
macro
{'f1-score': 0.6982787437381698,
 'precision': 0.7292296218608154,
 'recall': 0.6734733299965637,
 'support': 11150}
micro
{'f1-score': 0.9012556053811659,
 'precision': 0.9012556053811659,
 'recall': 0.9012556053811659,
 'support': 11150}
weighted
{'f1-score': 0.8964080931429855,
 'precision': 0.8933707245686591,
 'recall': 0.9012556053811659,
 'support': 11150}
### report_full
macro
{'f1-score': 0.6342173791652616,
 'precision': 0.7292296218608154,
 'recall': 0.5823128588595295,
 'support': 12421}
micro
{'f1-score': 0.8526579271138264,
 'precision': 0.9012556053811659,
 'recall': 0.809033089123259,
 'support': 12421}
weighted
{'f1-score': 0.8341815695488375,
 'precision': 0.8756517924581689,
 'recall': 0.809033089123259,
 'support': 12421}
## test
PPCR: 0.880597
### report
macro
{'f1-score': 0.6994474641114108,
 'precision': 0.7269202738170014,
 'recall': 0.6802087442955725,
 'support': 1003}
micro
{'f1-score': 0.8993020937188435,
 'precision': 0.8993020937188435,
 'recall': 0.8993020937188435,
 'support': 1003}
weighted
{'f1-score': 0.896084734965652,
 'precision': 0.8951691740726486,
 'recall': 0.8993020937188435,
 'support': 1003}
### report_full
macro
{'f1-score': 0.6113044810615447,
 'precision': 0.7269202738170014,
 'recall': 0.5550335786238139,
 'support': 1139}
micro
{'f1-score': 0.842203548085901,
 'precision': 0.8993020937188435,
 'recall': 0.7919227392449517,
 'support': 1139}
weighted
{'f1-score': 0.824669652481962,
 'precision': 0.8772785849991198,
 'recall': 0.7919227392449517,
 'support': 1139}
```

## javascript
### Summary
23 rules, avg.len. 4.7

| | |
|-|-|
|Min support|159|
|Max support|4216|
|Min confidence|0.9242172837257385|
|Max confidence|0.9988889098167419|

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
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 181,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 450.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>⇒ y = '<br>Confidence: 0.998. Support: 239.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 206.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.998. Support: 276.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_col ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 703.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.999. Support: 383.` |
| 7 | `  •••start_col ≥ 22<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {MAP}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.971. Support: 188.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.998. Support: 277.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ><br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.997. Support: 159.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.924. Support: 4216.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>⇒ y = '<br>Confidence: 0.998. Support: 253.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 305.` |
| 13 | `  •••start_col ≥ 21<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {VALUE}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.931. Support: 197.` |
| 14 | `  •••start_col ≤ 20<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.934. Support: 174.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 320.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 233.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.998. Support: 303.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.998. Support: 309.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.998. Support: 210.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 292.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_col ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 782.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 314.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 228.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.695652173913044, "max_conf": 0.9988889098167419, "max_support": 4216, "min_conf": 0.9242172837257385, "min_support": 159, "num_rules": 23}}
```
</details>
