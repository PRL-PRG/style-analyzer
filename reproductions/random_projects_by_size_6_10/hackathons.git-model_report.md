# Model report for file:///tmp/top-repos-quality-repos-ndrmo_t8/hackathons.git HEAD ce0349b5aa8807664b5fadc65ca5af1f855e8c25

### Dump

```json
{'created_at': '2021-08-21 11:14:31',
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
 'size': '13.6 kB',
 'tags': [],
 'uuid': 'b5546ce3-a562-44af-8628-b853b80a8f45',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ndrmo_t8/hackathons.git ce0349b5aa8807664b5fadc65ca5af1f855e8c25

# javascript
4 rules, avg.len. 4.0
## train
PPCR: 0.637883
### report
macro
{'f1-score': 0.37933466640514235,
 'precision': 0.387221396731055,
 'recall': 0.3737179487179487,
 'support': 2290}
micro
{'f1-score': 0.9436681222707424,
 'precision': 0.9436681222707424,
 'recall': 0.9436681222707424,
 'support': 2290}
weighted
{'f1-score': 0.9247540910657653,
 'precision': 0.9088393882569735,
 'recall': 0.9436681222707424,
 'support': 2290}
### report_full
macro
{'f1-score': 0.3390820652307089,
 'precision': 0.387221396731055,
 'recall': 0.30410627102638915,
 'support': 3590}
micro
{'f1-score': 0.7350340136054422,
 'precision': 0.9436681222707424,
 'recall': 0.601949860724234,
 'support': 3590}
weighted
{'f1-score': 0.6448841129656868,
 'precision': 0.6978208412835721,
 'recall': 0.601949860724234,
 'support': 3590}
## test
PPCR: 0.703665
### report
macro
{'f1-score': 0.39009082574782206,
 'precision': 0.3883408071748879,
 'recall': 0.39234042553191484,
 'support': 672}
micro
{'f1-score': 0.9613095238095238,
 'precision': 0.9613095238095238,
 'recall': 0.9613095238095238,
 'support': 672}
weighted
{'f1-score': 0.949110785321979,
 'precision': 0.9382674033739056,
 'recall': 0.9613095238095238,
 'support': 672}
### report_full
macro
{'f1-score': 0.36608551412588897,
 'precision': 0.3883408071748879,
 'recall': 0.3464321274477265,
 'support': 955}
micro
{'f1-score': 0.7940995697602949,
 'precision': 0.9613095238095238,
 'recall': 0.6764397905759162,
 'support': 955}
weighted
{'f1-score': 0.7114133982225617,
 'precision': 0.7506209940600568,
 'recall': 0.6764397905759162,
 'support': 955}
```

## javascript
### Summary
3 rules, avg.len. 3.3

| | |
|-|-|
|Min support|95|
|Max support|876|
|Min confidence|0.9823059439659119|
|Max confidence|0.9975845217704773|

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
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = CommentLine<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 207.` |
| 2 | `  -1.internal_type not in {CommentLine}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.982. Support: 876.` |
| 3 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved = .<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 95.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.3333333333333335, "max_conf": 0.9975845217704773, "max_support": 876, "min_conf": 0.9823059439659119, "min_support": 95, "num_rules": 3}}
```
</details>
