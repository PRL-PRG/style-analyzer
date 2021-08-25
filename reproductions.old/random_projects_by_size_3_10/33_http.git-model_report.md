# Model report for file:///tmp/top-repos-quality-repos-41lgoqd6/33_http.git HEAD 961662952bda88efe258c998bf31f23c5096e536

### Dump

```json
{'created_at': '2021-08-22 01:37:48',
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
 'size': '15.4 kB',
 'tags': [],
 'uuid': '8b92ee5a-e605-4af8-88fc-4b6a41e7b415',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-41lgoqd6/33_http.git 961662952bda88efe258c998bf31f23c5096e536

# javascript
9 rules, avg.len. 5.3
## train
PPCR: 0.580946
### report
macro
{'f1-score': 0.5295076312034358,
 'precision': 0.5361196571087495,
 'recall': 0.5234534779334596,
 'support': 2433}
micro
{'f1-score': 0.9605425400739828,
 'precision': 0.9605425400739828,
 'recall': 0.9605425400739828,
 'support': 2433}
weighted
{'f1-score': 0.9565738991935749,
 'precision': 0.9530007475496622,
 'recall': 0.9605425400739828,
 'support': 2433}
### report_full
macro
{'f1-score': 0.3775779647143924,
 'precision': 0.5361196571087495,
 'recall': 0.3251802705622117,
 'support': 4188}
micro
{'f1-score': 0.7059356592659719,
 'precision': 0.9605425400739828,
 'recall': 0.5580229226361032,
 'support': 4188}
weighted
{'f1-score': 0.6235391094884682,
 'precision': 0.8278595428840436,
 'recall': 0.5580229226361032,
 'support': 4188}
## test
PPCR: 0.235060
### report
macro
{'f1-score': 0.17616921768707483,
 'precision': 0.17169431875314228,
 'recall': 0.18386826096077197,
 'support': 472}
micro
{'f1-score': 0.9300847457627118,
 'precision': 0.9300847457627118,
 'recall': 0.9300847457627118,
 'support': 472}
weighted
{'f1-score': 0.9398286168280873,
 'precision': 0.9507535087046552,
 'recall': 0.9300847457627118,
 'support': 472}
### report_full
macro
{'f1-score': 0.0736286092099336,
 'precision': 0.17169431875314228,
 'recall': 0.048972389790947944,
 'support': 2008}
micro
{'f1-score': 0.35403225806451616,
 'precision': 0.9300847457627118,
 'recall': 0.21862549800796813,
 'support': 2008}
weighted
{'f1-score': 0.32745927084055193,
 'precision': 0.7052972123572077,
 'recall': 0.21862549800796816,
 'support': 2008}
```

## javascript
### Summary
7 rules, avg.len. 5.3

| | |
|-|-|
|Min support|95|
|Max support|705|
|Min confidence|0.9308943152427673|
|Max confidence|0.996268630027771|

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
| 1 | `  -1.roles in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.996. Support: 127.` |
| 2 | `  -1.roles not in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 705.` |
| 3 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 95.` |
| 4 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.938. Support: 104.` |
| 5 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 446.` |
| 6 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 134.` |
| 7 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 123.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.285714285714286, "max_conf": 0.996268630027771, "max_support": 705, "min_conf": 0.9308943152427673, "min_support": 95, "num_rules": 7}}
```
</details>
