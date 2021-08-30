# Model report for file:///tmp/top-repos-quality-repos-ma68wm4s/editly.git HEAD 6e69f4b33949502b9964c7f2826c07d2ffa20e1e

### Dump

```json
{'created_at': '2021-08-30 05:10:53',
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
 'size': '16.2 kB',
 'tags': [],
 'uuid': '1d3c9de0-8d4b-4288-87ef-4c87ca9301a9',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ma68wm4s/editly.git 6e69f4b33949502b9964c7f2826c07d2ffa20e1e

# javascript
20 rules, avg.len. 5.8
## train
PPCR: 0.875951
### report
macro
{'f1-score': 0.782149392701526,
 'precision': 0.7954253486061094,
 'recall': 0.7725966686769807,
 'support': 11630}
micro
{'f1-score': 0.9450558899398108,
 'precision': 0.9450558899398108,
 'recall': 0.9450558899398108,
 'support': 11630}
weighted
{'f1-score': 0.9432976441743649,
 'precision': 0.9425727079437516,
 'recall': 0.9450558899398108,
 'support': 11630}
### report_full
macro
{'f1-score': 0.7294206640225059,
 'precision': 0.7954253486061094,
 'recall': 0.6844720908084607,
 'support': 13277}
micro
{'f1-score': 0.8825631348616854,
 'precision': 0.9450558899398108,
 'recall': 0.8278225502749115,
 'support': 13277}
weighted
{'f1-score': 0.8658074670895121,
 'precision': 0.9128894514877649,
 'recall': 0.8278225502749115,
 'support': 13277}
## test
PPCR: 0.835783
### report
macro
{'f1-score': 0.6765611631680005,
 'precision': 0.739151276151756,
 'recall': 0.6355625420838275,
 'support': 1990}
micro
{'f1-score': 0.9150753768844221,
 'precision': 0.9150753768844221,
 'recall': 0.9150753768844221,
 'support': 1990}
weighted
{'f1-score': 0.9083626859136618,
 'precision': 0.9065515744954878,
 'recall': 0.9150753768844221,
 'support': 1990}
### report_full
macro
{'f1-score': 0.5662667955713605,
 'precision': 0.739151276151756,
 'recall': 0.4945651621758492,
 'support': 2381}
micro
{'f1-score': 0.8332189430336308,
 'precision': 0.9150753768844221,
 'recall': 0.7648047039059219,
 'support': 2381}
weighted
{'f1-score': 0.7996499977582268,
 'precision': 0.8768103313581584,
 'recall': 0.7648047039059219,
 'support': 2381}
```

## javascript
### Summary
12 rules, avg.len. 5.3

| | |
|-|-|
|Min support|95|
|Max support|3422|
|Min confidence|0.9229797720909119|
|Max confidence|0.9989878535270691|

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
| 1 | `  +1.length ≥ 2<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 410.` |
| 2 | `  -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.995. Support: 95.` |
| 3 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 408.` |
| 4 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 168.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 122.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = TemplateLiteral<br>⇒ y = ∅<br>Confidence: 0.995. Support: 110.` |
| 7 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {TemplateLiteral}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.963. Support: 174.` |
| 8 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {TemplateLiteral}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 1980.` |
| 9 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.999. Support: 494.` |
| 10 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>⇒ y = '<br>Confidence: 0.999. Support: 335.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 126.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 3422.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.333333333333333, "max_conf": 0.9989878535270691, "max_support": 3422, "min_conf": 0.9229797720909119, "min_support": 95, "num_rules": 12}}
```
</details>
