# Model report for file:///tmp/top-repos-quality-repos-52ykii9r/node-hid.git HEAD 2cdb2eb5060f1c861dd77d69575a505904cb400a

### Dump

```json
{'created_at': '2021-08-22 01:45:42',
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
 'size': '16.2 kB',
 'tags': [],
 'uuid': '5bd16eac-f7bf-4e7b-93ca-e856c5e98d62',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-52ykii9r/node-hid.git 2cdb2eb5060f1c861dd77d69575a505904cb400a

# javascript
4 rules, avg.len. 4.8
## train
PPCR: 0.427900
### report
macro
{'f1-score': 0.23512392594209514,
 'precision': 0.24288000543365212,
 'recall': 0.22864275665448902,
 'support': 1730}
micro
{'f1-score': 0.9757225433526011,
 'precision': 0.9757225433526011,
 'recall': 0.9757225433526011,
 'support': 1730}
weighted
{'f1-score': 0.9728020900232839,
 'precision': 0.9710656207965969,
 'recall': 0.9757225433526011,
 'support': 1730}
### report_full
macro
{'f1-score': 0.13534950679627653,
 'precision': 0.24288000543365212,
 'recall': 0.11035695412481973,
 'support': 4043}
micro
{'f1-score': 0.5847912697037936,
 'precision': 0.9757225433526011,
 'recall': 0.4175117487014593,
 'support': 4043}
weighted
{'f1-score': 0.49480360009140945,
 'precision': 0.7552870798462143,
 'recall': 0.4175117487014593,
 'support': 4043}
## test
PPCR: 0.438298
### report
macro
{'f1-score': 0.23359944463727872,
 'precision': 0.24344366536055545,
 'recall': 0.22572042319382746,
 'support': 412}
micro
{'f1-score': 0.9733009708737864,
 'precision': 0.9733009708737864,
 'recall': 0.9733009708737864,
 'support': 412}
weighted
{'f1-score': 0.9711214754855212,
 'precision': 0.9709615501411226,
 'recall': 0.9733009708737864,
 'support': 412}
### report_full
macro
{'f1-score': 0.13521205357142857,
 'precision': 0.24344366536055545,
 'recall': 0.10646862578643797,
 'support': 940}
micro
{'f1-score': 0.5931952662721894,
 'precision': 0.9733009708737864,
 'recall': 0.42659574468085104,
 'support': 940}
weighted
{'f1-score': 0.5204089095744682,
 'precision': 0.7912756011695034,
 'recall': 0.42659574468085104,
 'support': 940}
```

## javascript
### Summary
4 rules, avg.len. 4.8

| | |
|-|-|
|Min support|120|
|Max support|545|
|Min confidence|0.9495412707328796|
|Max confidence|0.9970414042472839|

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
                     'min_samples_leaf': 95,
                     'min_samples_split': 203,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 543.` |
| 2 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 120.` |
| 3 | `  -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 545.` |
| 4 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 169.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.75, "max_conf": 0.9970414042472839, "max_support": 545, "min_conf": 0.9495412707328796, "min_support": 120, "num_rules": 4}}
```
</details>
