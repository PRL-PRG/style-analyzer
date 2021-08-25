# Model report for file:///tmp/top-repos-quality-repos-d6c3j4rh/sound_of_sundholm.git HEAD bdd0df84422f79d4f010f393fd6ab36f4fe2a233

### Dump

```json
{'created_at': '2021-08-21 11:07:21',
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
 'size': '13.4 kB',
 'tags': [],
 'uuid': '834e95c4-f242-408b-832a-4122d0205cf4',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-d6c3j4rh/sound_of_sundholm.git bdd0df84422f79d4f010f393fd6ab36f4fe2a233

# javascript
2 rules, avg.len. 2.5
## train
PPCR: 0.501427
### report
macro
{'f1-score': 0.1927548441449031,
 'precision': 0.18601626016260162,
 'recall': 0.2,
 'support': 1230}
micro
{'f1-score': 0.9300813008130081,
 'precision': 0.9300813008130081,
 'recall': 0.9300813008130081,
 'support': 1230}
weighted
{'f1-score': 0.8963883809015006,
 'precision': 0.8650512261220173,
 'recall': 0.9300813008130081,
 'support': 1230}
### report_full
macro
{'f1-score': 0.17081000373273608,
 'precision': 0.18601626016260162,
 'recall': 0.15790200138026225,
 'support': 2453}
micro
{'f1-score': 0.6212326907412435,
 'precision': 0.9300813008130081,
 'recall': 0.4663677130044843,
 'support': 2453}
weighted
{'f1-score': 0.5044918373598342,
 'precision': 0.5494039155638193,
 'recall': 0.4663677130044843,
 'support': 2453}
## test
PPCR: 0.499598
### report
macro
{'f1-score': 0.19283930058284762,
 'precision': 0.18617363344051446,
 'recall': 0.2,
 'support': 622}
micro
{'f1-score': 0.9308681672025724,
 'precision': 0.9308681672025724,
 'recall': 0.9308681672025724,
 'support': 622}
weighted
{'f1-score': 0.8975398314909065,
 'precision': 0.8665155447110762,
 'recall': 0.9308681672025724,
 'support': 622}
### report_full
macro
{'f1-score': 0.17104874446085672,
 'precision': 0.18617363344051446,
 'recall': 0.1581967213114754,
 'support': 1245}
micro
{'f1-score': 0.6202463845741831,
 'precision': 0.9308681672025724,
 'recall': 0.4650602409638554,
 'support': 1245}
weighted
{'f1-score': 0.5028420921499884,
 'precision': 0.5473056211986209,
 'recall': 0.4650602409638554,
 'support': 1245}
```

## javascript
### Summary
1 rules, avg.len. 1.0

| | |
|-|-|
|Min support|441|
|Max support|441|
|Min confidence|0.9988662004470825|
|Max confidence|0.9988662004470825|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.999. Support: 441.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 1.0, "max_conf": 0.9988662004470825, "max_support": 441, "min_conf": 0.9988662004470825, "min_support": 441, "num_rules": 1}}
```
</details>
