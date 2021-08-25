# Model report for file:///tmp/top-repos-quality-repos-jidx60n4/listenserverc-.git HEAD 1306eb5c17f7baa90a3ac6a59110c2ff90b62872

### Dump

```json
{'created_at': '2021-08-24 10:16:13',
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
 'size': '18.4 kB',
 'tags': [],
 'uuid': '230f1bb1-56dc-494f-826d-aab317266189',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-jidx60n4/listenserverc-.git 1306eb5c17f7baa90a3ac6a59110c2ff90b62872

# javascript
10 rules, avg.len. 7.0
## train
PPCR: 0.782609
### report
macro
{'f1-score': 0.2722779844042223,
 'precision': 0.26805919336536654,
 'recall': 0.27674246003082714,
 'support': 10008}
micro
{'f1-score': 0.9320543565147882,
 'precision': 0.9320543565147882,
 'recall': 0.9320543565147882,
 'support': 10008}
weighted
{'f1-score': 0.9173287572291211,
 'precision': 0.9032603391672294,
 'recall': 0.9320543565147882,
 'support': 10008}
### report_full
macro
{'f1-score': 0.23765106033062025,
 'precision': 0.26805919336536654,
 'recall': 0.21754154173094387,
 'support': 12788}
micro
{'f1-score': 0.8183891910861554,
 'precision': 0.9320543565147882,
 'recall': 0.7294338442289646,
 'support': 12788}
weighted
{'f1-score': 0.7576288542879226,
 'precision': 0.792851884555793,
 'recall': 0.7294338442289646,
 'support': 12788}
## test
PPCR: 0.770338
### report
macro
{'f1-score': 0.25635467026890385,
 'precision': 0.24999612689985212,
 'recall': 0.26323884855027063,
 'support': 2301}
micro
{'f1-score': 0.9191655801825294,
 'precision': 0.9191655801825294,
 'recall': 0.9191655801825294,
 'support': 2301}
weighted
{'f1-score': 0.8979730302413712,
 'precision': 0.8779499142243342,
 'recall': 0.9191655801825294,
 'support': 2301}
### report_full
macro
{'f1-score': 0.22441891198063738,
 'precision': 0.24999612689985212,
 'recall': 0.2055404871257597,
 'support': 2987}
micro
{'f1-score': 0.7999243570347957,
 'precision': 0.9191655801825294,
 'recall': 0.7080682959491128,
 'support': 2987}
weighted
{'f1-score': 0.7415170312685311,
 'precision': 0.7815866814592627,
 'recall': 0.7080682959491128,
 'support': 2987}
```

## javascript
### Summary
5 rules, avg.len. 6.6

| | |
|-|-|
|Min support|154|
|Max support|2577|
|Min confidence|0.9305949211120605|
|Max confidence|0.9980079531669617|

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
| 1 | `  -1.reserved = ,<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 353.` |
| 2 | `  -1.reserved = var<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 251.` |
| 3 | `  •••start_col ≤ 34<br>	∧ -1.reserved not in {;, var, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.internal_type = Identifier<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 233.` |
| 4 | `  -1.reserved not in {,, ;, var, {}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 2577.` |
| 5 | `  -1.diff_col ≥ 2<br>	∧ -1.reserved = function<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 154.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.6, "max_conf": 0.9980079531669617, "max_support": 2577, "min_conf": 0.9305949211120605, "min_support": 154, "num_rules": 5}}
```
</details>
