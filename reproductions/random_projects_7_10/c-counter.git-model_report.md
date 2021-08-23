# Model report for file:///tmp/top-repos-quality-repos-kyshot0_/c-counter.git HEAD fdeabc2c1686484dafdae95b5cb0f33f59dfe504

### Dump

```json
{'created_at': '2021-08-21 06:16:16',
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
 'size': '12.3 kB',
 'tags': [],
 'uuid': 'be766e17-89c3-40f2-88ac-53d11675d37a',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-kyshot0_/c-counter.git fdeabc2c1686484dafdae95b5cb0f33f59dfe504

# javascript
3 rules, avg.len. 2.0
## train
PPCR: 0.571865
### report
macro
{'f1-score': 0.6158587035442312,
 'precision': 0.6076014633307962,
 'recall': 0.6264294457072915,
 'support': 748}
micro
{'f1-score': 0.9344919786096256,
 'precision': 0.9344919786096256,
 'recall': 0.9344919786096256,
 'support': 748}
weighted
{'f1-score': 0.9348648554936156,
 'precision': 0.9377005150648154,
 'recall': 0.9344919786096256,
 'support': 748}
### report_full
macro
{'f1-score': 0.46719343093965454,
 'precision': 0.6076014633307962,
 'recall': 0.37950407623893057,
 'support': 1308}
micro
{'f1-score': 0.6799610894941633,
 'precision': 0.9344919786096256,
 'recall': 0.5344036697247706,
 'support': 1308}
weighted
{'f1-score': 0.6585029575411793,
 'precision': 0.8576925779953918,
 'recall': 0.5344036697247706,
 'support': 1308}
## test
PPCR: 0.579310
### report
macro
{'f1-score': 0.5760590984471582,
 'precision': 0.5683347966052092,
 'recall': 0.5842661691542289,
 'support': 84}
micro
{'f1-score': 0.9047619047619048,
 'precision': 0.9047619047619048,
 'recall': 0.9047619047619048,
 'support': 84}
weighted
{'f1-score': 0.9000721500721499,
 'precision': 0.8956582633053222,
 'recall': 0.9047619047619048,
 'support': 84}
### report_full
macro
{'f1-score': 0.4121212121212121,
 'precision': 0.5683347966052092,
 'recall': 0.32832080200501257,
 'support': 145}
micro
{'f1-score': 0.663755458515284,
 'precision': 0.9047619047619048,
 'recall': 0.5241379310344828,
 'support': 145}
weighted
{'f1-score': 0.6400000000000001,
 'precision': 0.8359177742121039,
 'recall': 0.5241379310344828,
 'support': 145}
```

## javascript
### Summary
2 rules, avg.len. 2.5

| | |
|-|-|
|Min support|164|
|Max support|247|
|Min confidence|0.9359756112098694|
|Max confidence|0.9939271211624146|

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
                     'min_samples_leaf': 99,
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
| 1 | `  ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 247.` |
| 2 | `  -1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 164.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.5, "max_conf": 0.9939271211624146, "max_support": 247, "min_conf": 0.9359756112098694, "min_support": 164, "num_rules": 2}}
```
</details>
