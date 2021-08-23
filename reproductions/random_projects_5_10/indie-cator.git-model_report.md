# Model report for file:///tmp/top-repos-quality-repos-pfipibx8/indie-cator.git HEAD d707a506742a1eef9f5790dac152d5cb7c81a3c9

### Dump

```json
{'created_at': '2021-08-21 18:27:40',
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
 'size': '17.1 kB',
 'tags': [],
 'uuid': 'e73cb2a7-9cc6-40dd-8a97-6246ae22437b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-pfipibx8/indie-cator.git d707a506742a1eef9f5790dac152d5cb7c81a3c9

# javascript
15 rules, avg.len. 5.7
## train
PPCR: 0.939696
### report
macro
{'f1-score': 0.6756092272176004,
 'precision': 0.6921760428611995,
 'recall': 0.6648001372134307,
 'support': 14149}
micro
{'f1-score': 0.9232454590430419,
 'precision': 0.9232454590430419,
 'recall': 0.9232454590430419,
 'support': 14149}
weighted
{'f1-score': 0.9129193288913936,
 'precision': 0.9044161361368913,
 'recall': 0.9232454590430419,
 'support': 14149}
### report_full
macro
{'f1-score': 0.6501059577078178,
 'precision': 0.6921760428611995,
 'recall': 0.6179171808574639,
 'support': 15057}
micro
{'f1-score': 0.8945422173525988,
 'precision': 0.9232454590430419,
 'recall': 0.8675699010427044,
 'support': 15057}
weighted
{'f1-score': 0.8827163124200702,
 'precision': 0.9001644233333296,
 'recall': 0.8675699010427044,
 'support': 15057}
## test
PPCR: 0.932927
### report
macro
{'f1-score': 0.683594018307771,
 'precision': 0.709254139145088,
 'recall': 0.6638862897810485,
 'support': 1989}
micro
{'f1-score': 0.9421820010055304,
 'precision': 0.9421820010055304,
 'recall': 0.9421820010055304,
 'support': 1989}
weighted
{'f1-score': 0.9366366480558416,
 'precision': 0.9324575803002808,
 'recall': 0.9421820010055304,
 'support': 1989}
### report_full
macro
{'f1-score': 0.6470717416300421,
 'precision': 0.709254139145088,
 'recall': 0.6016608582886672,
 'support': 2132}
micro
{'f1-score': 0.9094879883523417,
 'precision': 0.9421820010055304,
 'recall': 0.8789868667917449,
 'support': 2132}
weighted
{'f1-score': 0.9021599692772391,
 'precision': 0.9325931145055759,
 'recall': 0.8789868667917449,
 'support': 2132}
```

## javascript
### Summary
8 rules, avg.len. 6.5

| | |
|-|-|
|Min support|102|
|Max support|4507|
|Min confidence|0.9228395223617554|
|Max confidence|0.9973404407501221|

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
                     'min_samples_leaf': 100,
                     'min_samples_split': 239,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.988. Support: 207.` |
| 2 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.970. Support: 252.` |
| 3 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 260.` |
| 4 | `  -1.reserved = ,<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.926. Support: 102.` |
| 5 | `  -1.reserved = ,<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 228.` |
| 6 | `  -1.reserved = var<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 188.` |
| 7 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.923. Support: 162.` |
| 8 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 4507.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.5, "max_conf": 0.9973404407501221, "max_support": 4507, "min_conf": 0.9228395223617554, "min_support": 102, "num_rules": 8}}
```
</details>
