# Model report for file:///tmp/top-repos-quality-repos-ry4jnrsx/infinity.git HEAD 2f38017af57363ef3b7f6d584264074b337eba22

### Dump

```json
{'created_at': '2021-08-30 05:14:18',
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
 'size': '14.8 kB',
 'tags': [],
 'uuid': 'c97aea20-e9e9-4a31-8d28-a2215297fe50',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ry4jnrsx/infinity.git 2f38017af57363ef3b7f6d584264074b337eba22

# javascript
7 rules, avg.len. 4.9
## train
PPCR: 0.829391
### report
macro
{'f1-score': 0.45556371317110494,
 'precision': 0.44429783704459896,
 'recall': 0.46780705708181713,
 'support': 3471}
micro
{'f1-score': 0.9242293287237108,
 'precision': 0.9242293287237108,
 'recall': 0.9242293287237108,
 'support': 3471}
weighted
{'f1-score': 0.912640848180292,
 'precision': 0.9020563676211512,
 'recall': 0.9242293287237108,
 'support': 3471}
### report_full
macro
{'f1-score': 0.40841362195413994,
 'precision': 0.44429783704459896,
 'recall': 0.3797478148100241,
 'support': 4185}
micro
{'f1-score': 0.8380355276907001,
 'precision': 0.9242293287237108,
 'recall': 0.766547192353644,
 'support': 4185}
weighted
{'f1-score': 0.8026336012601053,
 'precision': 0.8455349490761037,
 'recall': 0.766547192353644,
 'support': 4185}
## test
PPCR: 0.836421
### report
macro
{'f1-score': 0.46028049597794346,
 'precision': 0.44900871019712923,
 'recall': 0.47259249138535625,
 'support': 2935}
micro
{'f1-score': 0.9332197614991482,
 'precision': 0.9332197614991482,
 'recall': 0.9332197614991482,
 'support': 2935}
weighted
{'f1-score': 0.9221534623780363,
 'precision': 0.912217283741067,
 'recall': 0.9332197614991482,
 'support': 2935}
### report_full
macro
{'f1-score': 0.41409969840873395,
 'precision': 0.44900871019712923,
 'recall': 0.38622759797828493,
 'support': 3509}
micro
{'f1-score': 0.8500931098696461,
 'precision': 0.9332197614991482,
 'recall': 0.780564263322884,
 'support': 3509}
weighted
{'f1-score': 0.8149403601222202,
 'precision': 0.8558244968606559,
 'recall': 0.780564263322884,
 'support': 3509}
```

## javascript
### Summary
4 rules, avg.len. 6.8

| | |
|-|-|
|Min support|92|
|Max support|1284|
|Min confidence|0.929347813129425|
|Max confidence|0.9950980544090271|

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
                     'min_samples_split': 183,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.929. Support: 92.` |
| 2 | `  -1.reserved = )<br>	∧ -1.roles not in {COMMENT}<br>	∧ -3.reserved not in {)}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 102.` |
| 3 | `  -1.reserved not in {), ,, ;}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -2.label in {<newline>}<br>	∧ -3.reserved not in {)}<br>	∧ +2.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 418.` |
| 4 | `  -1.reserved not in {), ,, ;}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -2.label not in {<newline>}<br>	∧ -3.reserved not in {)}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 1284.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.75, "max_conf": 0.9950980544090271, "max_support": 1284, "min_conf": 0.929347813129425, "min_support": 92, "num_rules": 4}}
```
</details>
