# Model report for file:///tmp/top-repos-quality-repos-1s6pl1mw/nuk_projects.git HEAD d33f7d0147e45526ab884805dd599cfb651e36fd

### Dump

```json
{'created_at': '2021-08-21 19:34:27',
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
 'size': '15.3 kB',
 'tags': [],
 'uuid': 'a3b9ff8f-2384-46ec-9d44-d3d494155e0b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-1s6pl1mw/nuk_projects.git d33f7d0147e45526ab884805dd599cfb651e36fd

# javascript
6 rules, avg.len. 5.3
## train
PPCR: 0.568888
### report
macro
{'f1-score': 0.26022776240454604,
 'precision': 0.2653202526036888,
 'recall': 0.25594827779922064,
 'support': 3295}
micro
{'f1-score': 0.9426403641881639,
 'precision': 0.9426403641881639,
 'recall': 0.9426403641881639,
 'support': 3295}
weighted
{'f1-score': 0.9397501761697142,
 'precision': 0.9382641474738241,
 'recall': 0.9426403641881639,
 'support': 3295}
### report_full
macro
{'f1-score': 0.18934023600311117,
 'precision': 0.2653202526036888,
 'recall': 0.16120241314749323,
 'support': 5792}
micro
{'f1-score': 0.6836139540002201,
 'precision': 0.9426403641881639,
 'recall': 0.5362569060773481,
 'support': 5792}
weighted
{'f1-score': 0.6104451058300825,
 'precision': 0.7956637281454316,
 'recall': 0.5362569060773481,
 'support': 5792}
## test
PPCR: 0.506832
### report
macro
{'f1-score': 0.25709549154274747,
 'precision': 0.2584006412232567,
 'recall': 0.2558939526730938,
 'support': 408}
micro
{'f1-score': 0.9338235294117647,
 'precision': 0.9338235294117647,
 'recall': 0.9338235294117647,
 'support': 408}
weighted
{'f1-score': 0.9310389368206928,
 'precision': 0.9284635304272766,
 'recall': 0.9338235294117647,
 'support': 408}
### report_full
macro
{'f1-score': 0.18186667925597202,
 'precision': 0.2584006412232567,
 'recall': 0.15396031769793034,
 'support': 805}
micro
{'f1-score': 0.6281945589447651,
 'precision': 0.9338235294117647,
 'recall': 0.47329192546583854,
 'support': 805}
weighted
{'f1-score': 0.5456231911426025,
 'precision': 0.730449258384557,
 'recall': 0.47329192546583854,
 'support': 805}
```

## javascript
### Summary
3 rules, avg.len. 4.0

| | |
|-|-|
|Min support|223|
|Max support|848|
|Min confidence|0.9628537893295288|
|Max confidence|0.9932735562324524|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.990. Support: 759.` |
| 2 | `  +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 223.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {ARITHMETIC}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 848.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.0, "max_conf": 0.9932735562324524, "max_support": 848, "min_conf": 0.9628537893295288, "min_support": 223, "num_rules": 3}}
```
</details>
