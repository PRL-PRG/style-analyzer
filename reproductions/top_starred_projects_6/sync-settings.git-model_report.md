# Model report for file:///tmp/top-repos-quality-repos-60bwy3oa/sync-settings.git HEAD a3c22b57d613ef007d5f8fc78a260b1fa7a79bb0

### Dump

```json
{'created_at': '2021-08-29 21:20:12',
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
 'size': '17.2 kB',
 'tags': [],
 'uuid': '5c2930ae-2861-4438-a80e-985040eb7b59',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-60bwy3oa/sync-settings.git a3c22b57d613ef007d5f8fc78a260b1fa7a79bb0

# javascript
86 rules, avg.len. 6.7
## train
PPCR: 0.974550
### report
macro
{'f1-score': 0.7760467096554137,
 'precision': 0.7862581112162974,
 'recall': 0.7679181785834694,
 'support': 23780}
micro
{'f1-score': 0.9309924306139613,
 'precision': 0.9309924306139613,
 'recall': 0.9309924306139613,
 'support': 23780}
weighted
{'f1-score': 0.9280569702346808,
 'precision': 0.9263559809071928,
 'recall': 0.9309924306139613,
 'support': 23780}
### report_full
macro
{'f1-score': 0.7648213764898194,
 'precision': 0.7862581112162974,
 'recall': 0.7491598187974791,
 'support': 24401}
micro
{'f1-score': 0.9189929640314647,
 'precision': 0.9309924306139613,
 'recall': 0.9072988811933937,
 'support': 24401}
weighted
{'f1-score': 0.9089388786845299,
 'precision': 0.9130816731330362,
 'recall': 0.9072988811933937,
 'support': 24401}
## test
PPCR: 0.973960
### report
macro
{'f1-score': 0.6982235776914025,
 'precision': 0.6858813094397533,
 'recall': 0.7209482718403468,
 'support': 4825}
micro
{'f1-score': 0.823419689119171,
 'precision': 0.823419689119171,
 'recall': 0.823419689119171,
 'support': 4825}
weighted
{'f1-score': 0.8285612406649423,
 'precision': 0.8435638019524535,
 'recall': 0.823419689119171,
 'support': 4825}
### report_full
macro
{'f1-score': 0.6757437454575805,
 'precision': 0.6858813094397533,
 'recall': 0.6824772714736698,
 'support': 4954}
micro
{'f1-score': 0.8125575212189384,
 'precision': 0.823419689119171,
 'recall': 0.8019781994348002,
 'support': 4954}
weighted
{'f1-score': 0.8133684667898898,
 'precision': 0.8367494265082147,
 'recall': 0.8019781994348002,
 'support': 4954}
```

## javascript
### Summary
55 rules, avg.len. 6.0

| | |
|-|-|
|Min support|207|
|Max support|4866|
|Min confidence|0.9200913310050964|
|Max confidence|0.9994487166404724|

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
                     'min_samples_leaf': 120,
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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.974. Support: 4735.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 897.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 381.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 3218.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 460.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1572.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.926. Support: 740.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 386.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 354.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 322.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.997. Support: 535.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {BlockStatement, MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 210.` |
| 13 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 4811.` |
| 14 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 890.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 438.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 790.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 373.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 624.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 3016.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 409.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 1734.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 427.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 772.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 397.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 624.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 707.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 2794.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 240.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 1648.` |
| 30 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 4866.` |
| 31 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 907.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 351.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 3143.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 435.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1517.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 413.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 373.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 307.` |
| 39 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 530.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 207.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 401.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 3180.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 466.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1538.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.920. Support: 657.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 394.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 344.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 330.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.roles in {SCOPE} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 550.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 348.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 3078.` |
| 52 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.roles in {BLOCK} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 563.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ><br>	∧ ^1.roles not in {BLOCK, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 207.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 3042.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 346.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.036363636363636, "max_conf": 0.9994487166404724, "max_support": 4866, "min_conf": 0.9200913310050964, "min_support": 207, "num_rules": 55}}
```
</details>
