# Model report for file:///tmp/top-repos-quality-repos-ibuxsox4/raspberrycast.git HEAD 1976dae9cfb5f17fe72acc7bf3ec9a7fe9b8ecd8

### Dump

```json
{'created_at': '2021-08-29 14:14:55',
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
 'uuid': 'c15a848d-6cfa-407f-9798-411ec8940bc8',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ibuxsox4/raspberrycast.git 1976dae9cfb5f17fe72acc7bf3ec9a7fe9b8ecd8

# javascript
5 rules, avg.len. 3.8
## train
PPCR: 0.545712
### report
macro
{'f1-score': 0.7540399111124442,
 'precision': 0.7562304136405786,
 'recall': 0.7670056697902327,
 'support': 1928}
micro
{'f1-score': 0.9190871369294605,
 'precision': 0.9190871369294605,
 'recall': 0.9190871369294605,
 'support': 1928}
weighted
{'f1-score': 0.9065543173164371,
 'precision': 0.9049220563645437,
 'recall': 0.9190871369294605,
 'support': 1928}
### report_full
macro
{'f1-score': 0.5878818195985069,
 'precision': 0.7562304136405786,
 'recall': 0.5437102912201762,
 'support': 3533}
micro
{'f1-score': 0.6489653909540378,
 'precision': 0.9190871369294605,
 'recall': 0.5015567506368526,
 'support': 3533}
weighted
{'f1-score': 0.6118132528288449,
 'precision': 0.8950828716955702,
 'recall': 0.5015567506368526,
 'support': 3533}
## test
PPCR: 0.605769
### report
macro
{'f1-score': 0.7577709417607116,
 'precision': 0.7718478764990393,
 'recall': 0.7547862575399948,
 'support': 378}
micro
{'f1-score': 0.9312169312169312,
 'precision': 0.9312169312169312,
 'recall': 0.9312169312169312,
 'support': 378}
weighted
{'f1-score': 0.9261128906141696,
 'precision': 0.9283745890119706,
 'recall': 0.9312169312169312,
 'support': 378}
### report_full
macro
{'f1-score': 0.6164001666750577,
 'precision': 0.7718478764990393,
 'recall': 0.5609382967873534,
 'support': 624}
micro
{'f1-score': 0.7025948103792415,
 'precision': 0.9312169312169312,
 'recall': 0.5641025641025641,
 'support': 624}
weighted
{'f1-score': 0.6728781999644775,
 'precision': 0.8970902838299976,
 'recall': 0.5641025641025641,
 'support': 624}
```

## javascript
### Summary
2 rules, avg.len. 2.5

| | |
|-|-|
|Min support|145|
|Max support|179|
|Min confidence|0.934482753276825|
|Max confidence|0.9972066879272461|

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
                     'max_depth': 10,
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
| 1 | `  -1.roles in {STRING}<br>⇒ y = "<br>Confidence: 0.997. Support: 179.` |
| 2 | `  -1.reserved not in {{}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 145.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.5, "max_conf": 0.9972066879272461, "max_support": 179, "min_conf": 0.934482753276825, "min_support": 145, "num_rules": 2}}
```
</details>
